defmodule Lumis.Stress.CorpusRunner do
  @moduledoc false

  @repo_root Path.expand("../../../..", __DIR__)

  @defaults [
    manifest: "../../../../target/stress-test/corpus/manifest.json",
    output: "../../../../target/stress-test/report.json",
    profile: "all",
    iterations: 1,
    max_case_ms: 30_000,
    max_output_amplification: 32.0,
    max_probe_ms: 5_000,
    caller_timeout_ms: 1_000,
    timeout_storm: false,
    characterize: false
  ]

  def main(argv) do
    options = parse_options!(argv)
    manifest = load_manifest!(options)
    cases = select_cases!(manifest.cases, options)
    output_path = Path.expand(options[:output], __DIR__)
    File.mkdir_p!(Path.dirname(output_path))

    languages = cases |> Enum.map(& &1.language) |> Enum.uniq() |> Enum.sort()
    preload = timed(fn -> Lumis.Languages.load(languages) end)

    report = %{
      schema_version: 1,
      status: "running",
      generated_at: timestamp(),
      completed_at: nil,
      running_case: nil,
      runtime: runtime(),
      options: Map.new(options),
      corpus: %{
        scale: manifest.scale,
        discovery: manifest.discovery,
        cases: Enum.map(cases, &Map.drop(&1, [:generated_path])),
        languages: languages,
        origin_files: cases |> Enum.flat_map(& &1.origins) |> length()
      },
      preload: preload,
      results: [],
      timeout_storm: nil,
      violations: []
    }

    write_report!(output_path, report)
    ensure_preload!(preload)

    report =
      Enum.reduce(cases, report, fn test_case, current ->
        write_report!(output_path, %{current | running_case: test_case.id})
        result = run_case(test_case, options)
        IO.puts(format_result(result))

        updated = %{
          current
          | generated_at: timestamp(),
            running_case: nil,
            results: current.results ++ [result]
        }

        write_report!(output_path, updated)
        updated
      end)

    report = maybe_run_timeout_storm(report, cases, options, output_path)
    violations = violations(report, options)

    final = %{
      report
      | status: if(violations == [], do: "ok", else: "failed"),
        generated_at: timestamp(),
        completed_at: timestamp(),
        running_case: nil,
        violations: violations
    }

    write_report!(output_path, final)
    print_summary(final, output_path)

    if final.status == "failed" and not options[:characterize], do: System.halt(2)
  end

  defp parse_options!(argv) do
    argv = if List.first(argv) == "--", do: tl(argv), else: argv

    {parsed, rest, invalid} =
      OptionParser.parse(argv,
        strict: [
          case: [:string, :keep],
          profile: :string,
          manifest: :string,
          output: :string,
          iterations: :integer,
          max_case_ms: :integer,
          max_output_amplification: :float,
          max_probe_ms: :integer,
          caller_timeout_ms: :integer,
          storm_callers: :integer,
          timeout_storm: :boolean,
          characterize: :boolean
        ]
      )

    if rest != [] or invalid != [] do
      raise ArgumentError, "invalid arguments: #{inspect(rest ++ invalid)}"
    end

    options = Keyword.merge(@defaults, parsed)

    if options[:iterations] < 1,
      do: raise(ArgumentError, "--iterations must be at least one")

    options
  end

  defp load_manifest!(options) do
    path = Path.expand(options[:manifest], __DIR__)
    decoded = path |> File.read!() |> Jason.decode!()

    %{
      scale: decoded["scale"],
      discovery: decoded["discovery"],
      cases: Enum.map(decoded["cases"], &normalize_case/1)
    }
  end

  defp normalize_case(test_case) do
    generated = test_case["generated"]

    %{
      id: test_case["id"],
      profile: test_case["profile"],
      language: test_case["language"],
      target: test_case["target"],
      generated: %{
        bytes: generated["bytes"],
        lines: generated["lines"],
        max_line_bytes: generated["maxLineBytes"]
      },
      generated_path: Path.expand(test_case["generatedPath"], @repo_root),
      source_sha256: test_case["sourceSha256"],
      origins: test_case["origins"],
      scale: test_case["scale"]
    }
  end

  defp select_cases!(cases, options) do
    requested_ids = Keyword.get_values(options, :case)
    profile = options[:profile]

    selected =
      cases
      |> Enum.filter(fn test_case ->
        (requested_ids == [] or test_case.id in requested_ids) and
          (profile == "all" or test_case.profile == profile)
      end)

    known_ids = MapSet.new(cases, & &1.id)
    unknown_ids = Enum.reject(requested_ids, &MapSet.member?(known_ids, &1))

    cond do
      unknown_ids != [] ->
        raise ArgumentError, "unknown corpus cases: #{Enum.join(unknown_ids, ", ")}"

      selected == [] ->
        raise ArgumentError, "no corpus cases matched profile #{inspect(profile)}"

      true ->
        selected
    end
  end

  defp run_case(test_case, options) do
    source = File.read!(test_case.generated_path)
    ensure_generated_source!(test_case, source)

    iterations =
      for iteration <- 1..options[:iterations] do
        measurement =
          measure(fn ->
            Lumis.highlight(source,
              formatter: {:html_linked, language: test_case.language}
            )
          end)

        compact_measurement(iteration, measurement)
      end

    successes = Enum.filter(iterations, &(&1.status == "ok"))
    output_hashes = Enum.map(successes, & &1.output_sha256)
    output_bytes = successes |> Enum.map(& &1.output_bytes) |> Enum.max(fn -> nil end)

    %{
      id: test_case.id,
      profile: test_case.profile,
      language: test_case.language,
      scale: test_case.scale,
      target: test_case.target,
      generated: test_case.generated,
      source_sha256: test_case.source_sha256,
      origins: test_case.origins,
      status: if(length(successes) == length(iterations), do: "ok", else: "error"),
      deterministic: determinism(output_hashes),
      output_bytes: output_bytes,
      output_amplification:
        if(output_bytes, do: output_bytes / max(byte_size(source), 1), else: nil),
      iterations: iterations
    }
  rescue
    exception ->
      %{
        id: test_case.id,
        profile: test_case.profile,
        language: test_case.language,
        status: "error",
        error: Exception.format(:error, exception, __STACKTRACE__),
        origins: test_case.origins
      }
  catch
    kind, reason ->
      %{
        id: test_case.id,
        profile: test_case.profile,
        language: test_case.language,
        status: "error",
        error: inspect({kind, reason}),
        origins: test_case.origins
      }
  end

  # `nil` when fewer than two renders were compared, so a report never claims a
  # determinism it did not check. Only `false` is a violation.
  defp determinism(hashes) when length(hashes) < 2, do: nil
  defp determinism(hashes), do: length(Enum.uniq(hashes)) == 1

  defp compact_measurement(iteration, %{value: {:ok, output}} = measurement) do
    measurement
    |> Map.drop([:value])
    |> Map.merge(%{
      iteration: iteration,
      status: "ok",
      output_bytes: byte_size(output),
      output_sha256: sha256(output)
    })
  end

  defp compact_measurement(iteration, %{value: {:error, reason}} = measurement) do
    measurement
    |> Map.drop([:value])
    |> Map.merge(%{iteration: iteration, status: "error", error: inspect(reason)})
  end

  defp maybe_run_timeout_storm(report, cases, options, output_path) do
    if options[:timeout_storm] do
      run_timeout_storm(report, cases, options, output_path)
    else
      report
    end
  end

  defp run_timeout_storm(report, cases, options, output_path) do
    deep_case =
      Enum.find(cases, &(&1.id == "deep-json")) ||
        raise ArgumentError, "--timeout-storm requires the deep-json case"

    source = File.read!(deep_case.generated_path)
    ensure_generated_source!(deep_case, source)
    callers = options[:storm_callers] || :erlang.system_info(:dirty_cpu_schedulers) * 2

    write_report!(output_path, %{report | running_case: "timeout-storm"})
    {:ok, supervisor} = Task.Supervisor.start_link()
    caller_batch = measure(fn -> run_timed_callers(supervisor, source, callers, options) end)

    probe =
      compact_measurement(
        1,
        measure(fn ->
          Lumis.highlight("{}\n", formatter: {:html_linked, language: "json"})
        end)
      )

    storm = %{
      callers: callers,
      caller_timeout_ms: options[:caller_timeout_ms],
      caller_wall_ms: caller_batch.wall_ms,
      timed_out_callers: Enum.count(caller_batch.value, &(&1 == :timeout)),
      completed_callers: Enum.count(caller_batch.value, &match?({:ok, _}, &1)),
      failed_callers: Enum.count(caller_batch.value, &match?({:error, _}, &1)),
      memory: caller_batch.memory,
      immediate_probe: probe
    }

    updated = %{report | generated_at: timestamp(), running_case: nil, timeout_storm: storm}
    write_report!(output_path, updated)
    # The ignored callers are still under this supervisor and still inside the
    # NIF, so stopping it waits for them. The probe is already on disk by then.
    Supervisor.stop(supervisor)
    updated
  end

  defp run_timed_callers(supervisor, source, callers, options) do
    1..callers
    |> Task.async_stream(
      fn _ -> timed_call(supervisor, source, options[:caller_timeout_ms]) end,
      max_concurrency: callers,
      ordered: false,
      timeout: :infinity
    )
    |> Enum.map(fn
      {:ok, result} -> result
      {:exit, reason} -> {:error, reason}
    end)
  end

  defp timed_call(supervisor, source, timeout_ms) do
    task =
      Task.Supervisor.async_nolink(supervisor, fn ->
        Lumis.highlight(source, formatter: {:html_linked, language: "json"})
      end)

    case Task.yield(task, timeout_ms) do
      {:ok, result} ->
        {:ok, result}

      {:exit, reason} ->
        {:error, reason}

      nil ->
        # `Task.shutdown/2` waits for the `:DOWN`, and a process inside a DirtyCpu
        # NIF cannot be killed until the NIF returns. Waiting here is exactly the
        # symptom this probe measures, so the caller gives up and leaves the native
        # work occupying its scheduler.
        Task.ignore(task)
        :timeout
    end
  end

  defp violations(report, options) do
    Enum.flat_map(report.results, &case_violations(&1, options)) ++
      storm_violations(report.timeout_storm, options)
  end

  defp case_violations(%{status: status, id: id}, _options) when status != "ok",
    do: ["#{id}: render failed"]

  defp case_violations(result, options) do
    nondeterministic =
      if result.deterministic == false,
        do: ["#{result.id}: output was nondeterministic"],
        else: []

    slow =
      result.iterations
      |> Enum.filter(&(&1.wall_ms > options[:max_case_ms]))
      |> Enum.map(&slow_violation(result.id, &1, options[:max_case_ms]))

    nondeterministic ++
      slow ++ amplification_violation(result, options[:max_output_amplification])
  end

  defp slow_violation(id, measurement, budget) do
    "#{id}: iteration #{measurement.iteration} took #{measurement.wall_ms} ms " <>
      "(budget #{budget} ms)"
  end

  defp amplification_violation(result, budget) when result.output_amplification > budget do
    [
      "#{result.id}: output amplification #{Float.round(result.output_amplification, 2)}x " <>
        "(budget #{budget}x)"
    ]
  end

  defp amplification_violation(_result, _budget), do: []

  defp storm_violations(nil, _options), do: []

  defp storm_violations(storm, options) do
    probe_ms = storm.immediate_probe.wall_ms

    if probe_ms > options[:max_probe_ms] do
      [
        "timeout-storm: immediate tiny probe took #{probe_ms} ms " <>
          "after callers returned (budget #{options[:max_probe_ms]} ms)"
      ]
    else
      []
    end
  end

  defp measure(fun) do
    baseline = memory_snapshot()
    caller = self()
    sampler = spawn_link(fn -> sample_memory(caller, baseline) end)
    started = System.monotonic_time()

    try do
      value = fun.()
      collect_measurement(sampler, baseline, elapsed_ms(started), value)
    catch
      kind, reason ->
        stacktrace = __STACKTRACE__
        # `run_case/2` rescues in this process, so the link never fires and the
        # sampler would recurse forever on its own timeout.
        collect_measurement(sampler, baseline, elapsed_ms(started), nil)
        :erlang.raise(kind, reason, stacktrace)
    end
  end

  defp collect_measurement(sampler, baseline, wall_ms, value) do
    send(sampler, {:stop, self()})

    peak =
      receive do
        {:memory_peak, ^sampler, snapshot} -> snapshot
      after
        1_000 -> memory_snapshot()
      end

    %{
      value: value,
      wall_ms: wall_ms,
      memory: %{
        before: baseline,
        peak: peak,
        after: memory_snapshot(),
        beam_peak_delta_bytes: peak.beam_bytes - baseline.beam_bytes,
        rss_peak_delta_kb: subtract(peak.rss_kb, baseline.rss_kb)
      }
    }
  end

  defp sample_memory(caller, peak) do
    receive do
      # Sampling here too, because a render finishing inside the first interval
      # would otherwise report the baseline as its peak.
      {:stop, ^caller} ->
        send(caller, {:memory_peak, self(), merge_peak(peak, memory_snapshot())})
    after
      50 -> sample_memory(caller, merge_peak(peak, memory_snapshot()))
    end
  end

  defp merge_peak(peak, current) do
    %{
      beam_bytes: max(peak.beam_bytes, current.beam_bytes),
      rss_kb: max_optional(peak.rss_kb, current.rss_kb)
    }
  end

  defp memory_snapshot do
    %{beam_bytes: :erlang.memory(:total), rss_kb: rss_kb()}
  end

  defp rss_kb do
    status_path = "/proc/#{System.pid()}/status"

    with {:ok, status} <- File.read(status_path),
         [_, value] <- Regex.run(~r/^VmRSS:\s+(\d+)\s+kB$/m, status) do
      String.to_integer(value)
    else
      _ -> nil
    end
  end

  defp max_optional(nil, value), do: value
  defp max_optional(value, nil), do: value
  defp max_optional(left, right), do: max(left, right)

  defp subtract(nil, _right), do: nil
  defp subtract(_left, nil), do: nil
  defp subtract(left, right), do: left - right

  defp timed(fun) do
    started = System.monotonic_time()
    result = fun.()
    %{result: inspect(result), wall_ms: elapsed_ms(started)}
  end

  defp elapsed_ms(started) do
    System.monotonic_time()
    |> Kernel.-(started)
    |> System.convert_time_unit(:native, :millisecond)
  end

  defp ensure_preload!(%{result: ":ok"}), do: :ok
  defp ensure_preload!(preload), do: raise("failed to load stress languages: #{preload.result}")

  defp runtime do
    %{
      elixir: System.version(),
      otp_release: System.otp_release(),
      schedulers_online: :erlang.system_info(:schedulers_online),
      dirty_cpu_schedulers: :erlang.system_info(:dirty_cpu_schedulers),
      lumis: :lumis |> Application.spec(:vsn) |> to_string(),
      mix_env: Mix.env(),
      git_revision: git_revision()
    }
  end

  defp git_revision do
    case System.cmd("git", ["rev-parse", "HEAD"], stderr_to_stdout: true) do
      {revision, 0} -> String.trim(revision)
      _ -> nil
    end
  end

  defp format_result(result) when result.status == "ok" do
    slowest = result.iterations |> Enum.map(& &1.wall_ms) |> Enum.max()

    "#{result.id}: #{slowest} ms, #{result.generated.bytes} source bytes, " <>
      "#{result.output_bytes} output bytes"
  end

  defp format_result(result), do: "#{result.id}: #{result.status}"

  defp print_summary(report, output_path) do
    IO.puts("Wrote #{length(report.results)} stress results to #{output_path}")

    Enum.each(report.violations, &IO.puts(:stderr, "VIOLATION: #{&1}"))
  end

  defp write_report!(path, report) do
    temporary = path <> ".tmp"
    File.write!(temporary, Jason.encode_to_iodata!(json_value(report), pretty: true))
    File.rename!(temporary, path)
  end

  defp json_value(value) when is_map(value) do
    Map.new(value, fn {key, nested} -> {json_key(key), json_value(nested)} end)
  end

  defp json_value(value) when is_list(value), do: Enum.map(value, &json_value/1)
  defp json_value(value), do: value

  defp json_key(key) when is_binary(key), do: key

  defp json_key(key) when is_atom(key) do
    [first | rest] = key |> Atom.to_string() |> String.split("_")
    first <> Enum.map_join(rest, &String.capitalize/1)
  end

  defp sha256(bytes) do
    :crypto.hash(:sha256, bytes) |> Base.encode16(case: :lower)
  end

  defp ensure_generated_source!(test_case, source) do
    if byte_size(source) != test_case.generated.bytes or sha256(source) != test_case.source_sha256 do
      raise "generated bytes changed for #{test_case.id}"
    end
  end

  defp timestamp, do: DateTime.utc_now() |> DateTime.to_iso8601()
end

Lumis.Stress.CorpusRunner.main(System.argv())
