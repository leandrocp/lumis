import Config

if config_env() == :test do
  # `mix test` runs `stage.test_parsers` first, which lays the committed parser
  # fixtures out as a real language store. The suite reads it in place, so it
  # exercises the real resolve-verify-load path without a download and without
  # copying anything. This cannot live in test_helper.exs: it is applied before
  # the store is built, and `System.put_env` is invisible to the NIF.
  data_dir = Path.expand("../../../../target/test-parsers", __DIR__)
  config :lumis, data_dir: data_dir

  # The suite depends on no parser packages, so without this it would declare
  # nothing and load nothing. Pointing at the staged fixtures is exactly what a
  # project vendoring its parsers does, which keeps the tests on the real
  # declared-set path rather than an escape hatch only tests take.
  config :lumis, parser_dirs: [Path.join(data_dir, "parsers")]
end
