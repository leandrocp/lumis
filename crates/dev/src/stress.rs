use anyhow::{bail, Context, Result};
use lumis::{highlight, languages::Language, HtmlLinkedBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub(crate) struct Options {
    pub(crate) manifest: PathBuf,
    pub(crate) output: PathBuf,
    pub(crate) iterations: usize,
    pub(crate) max_case_ms: u128,
    pub(crate) max_output_amplification: f64,
    pub(crate) characterize: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CorpusManifest {
    schema_version: u64,
    scale: f64,
    discovery: Value,
    cases: Vec<StressCase>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct StressCase {
    id: String,
    profile: String,
    language: String,
    target: Value,
    generated: GeneratedMetrics,
    generated_path: PathBuf,
    source_sha256: String,
    origins: Vec<Value>,
    scale: f64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeneratedMetrics {
    bytes: usize,
    lines: usize,
    max_line_bytes: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Report {
    schema_version: u64,
    runtime: Runtime,
    status: String,
    generated_at_unix_ms: u128,
    completed_at_unix_ms: Option<u128>,
    running_case: Option<String>,
    options: ReportOptions,
    corpus: CorpusSummary,
    preload: Option<Preload>,
    results: Vec<CaseResult>,
    violations: Vec<String>,
}

/// Every other lane records query compilation separately. Without this the
/// first iteration of each language pays for forcing its `LazyLock`
/// `HighlightConfiguration` and the rest do not, which makes the per-lane
/// timing comparison this suite exists for a cold number against warm ones.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Preload {
    languages: Vec<String>,
    wall_ms: u128,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Runtime {
    id: &'static str,
    implementation: &'static str,
    operating_system: &'static str,
    architecture: &'static str,
    git_revision: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportOptions {
    iterations: usize,
    max_case_ms: u128,
    max_output_amplification: f64,
    characterize: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CorpusSummary {
    scale: f64,
    discovery: Value,
    cases: Vec<StressCase>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CaseResult {
    id: String,
    profile: String,
    language: String,
    status: &'static str,
    generated: GeneratedMetrics,
    source_sha256: String,
    origins: Vec<Value>,
    deterministic: Option<bool>,
    output_bytes: usize,
    output_amplification: f64,
    iterations: Vec<Iteration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Iteration {
    #[serde(rename = "iteration")]
    number: usize,
    wall_ms: u128,
    output_bytes: usize,
    output_sha256: String,
    memory: Memory,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Memory {
    #[serde(rename = "beforeRssKb")]
    before_rss: Option<u64>,
    #[serde(rename = "peakRssKb")]
    peak_rss: Option<u64>,
    #[serde(rename = "afterRssKb")]
    after_rss: Option<u64>,
    #[serde(rename = "peakDeltaKb")]
    peak_delta: Option<u64>,
}

pub(crate) fn run(options: Options) -> Result<()> {
    if options.iterations == 0 {
        bail!("--iterations must be at least one");
    }
    // A NaN budget makes every `>` comparison false, so the run would report
    // "ok" having checked nothing.
    if !options.max_output_amplification.is_finite() || options.max_output_amplification < 0.0 {
        bail!("--max-output-amplification must be a finite non-negative number");
    }

    let manifest: CorpusManifest = serde_json::from_slice(
        &fs::read(&options.manifest)
            .with_context(|| format!("read {}", options.manifest.display()))?,
    )?;
    let mut report = initial_report(&manifest, &options);
    write_report(&options.output, &report)?;

    report.preload = Some(preload(&manifest)?);
    write_report(&options.output, &report)?;

    for test_case in &manifest.cases {
        report.running_case = Some(test_case.id.clone());
        report.generated_at_unix_ms = unix_ms();
        write_report(&options.output, &report)?;

        let result = run_case(test_case, options.iterations)?;
        println!(
            "{}: {} ms, {} source bytes, {} output bytes",
            result.id,
            result
                .iterations
                .iter()
                .map(|iteration| iteration.wall_ms)
                .max()
                .unwrap_or_default(),
            result.generated.bytes,
            result.output_bytes
        );
        report.results.push(result);
        report.running_case = None;
        report.generated_at_unix_ms = unix_ms();
        write_report(&options.output, &report)?;
    }

    report.violations = violations(&report.results, &options);
    report.status = if report.violations.is_empty() {
        "ok".to_string()
    } else {
        "failed".to_string()
    };
    report.completed_at_unix_ms = Some(unix_ms());
    report.generated_at_unix_ms = unix_ms();
    write_report(&options.output, &report)?;

    for violation in &report.violations {
        eprintln!("VIOLATION: {violation}");
    }
    if !report.violations.is_empty() && !options.characterize {
        bail!("stress-test budgets failed");
    }
    Ok(())
}

fn initial_report(manifest: &CorpusManifest, options: &Options) -> Report {
    Report {
        schema_version: 1,
        runtime: Runtime {
            id: "rust",
            implementation: "native-tree-sitter",
            operating_system: std::env::consts::OS,
            architecture: std::env::consts::ARCH,
            git_revision: git_revision(),
        },
        status: "running".to_string(),
        generated_at_unix_ms: unix_ms(),
        completed_at_unix_ms: None,
        running_case: None,
        options: ReportOptions {
            iterations: options.iterations,
            max_case_ms: options.max_case_ms,
            max_output_amplification: options.max_output_amplification,
            characterize: options.characterize,
        },
        corpus: CorpusSummary {
            scale: manifest.scale,
            discovery: manifest.discovery.clone(),
            cases: manifest.cases.clone(),
        },
        preload: None,
        results: Vec::new(),
        violations: Vec::new(),
    }
}

/// Highlighting one byte per language forces that language's `LazyLock`
/// `HighlightConfiguration`, the same work `lumis languages download`,
/// `Lumis.Languages.load/1` and `createHighlighter` are timed for elsewhere.
fn preload(manifest: &CorpusManifest) -> Result<Preload> {
    let mut languages: Vec<String> = manifest
        .cases
        .iter()
        .map(|test_case| test_case.language.clone())
        .collect();
    languages.sort_unstable();
    languages.dedup();

    let started = Instant::now();
    for name in &languages {
        let language = name
            .parse::<Language>()
            .map_err(|_| anyhow::anyhow!("invalid language '{name}'"))?;
        let formatter = HtmlLinkedBuilder::new()
            .language(language)
            .build()
            .map_err(|error| anyhow::anyhow!("{error}"))?;
        highlight("", formatter);
    }

    Ok(Preload {
        languages,
        wall_ms: started.elapsed().as_millis(),
    })
}

fn run_case(test_case: &StressCase, iterations: usize) -> Result<CaseResult> {
    let source = fs::read_to_string(&test_case.generated_path)
        .with_context(|| format!("read {}", test_case.generated_path.display()))?;
    let source_hash = sha256(source.as_bytes());
    if source.len() != test_case.generated.bytes || source_hash != test_case.source_sha256 {
        bail!("generated bytes changed for {}", test_case.id);
    }

    let language = test_case
        .language
        .parse::<Language>()
        .map_err(|_| anyhow::anyhow!("invalid language '{}'", test_case.language))?;
    let mut measurements = Vec::with_capacity(iterations);
    for iteration in 1..=iterations {
        let formatter = HtmlLinkedBuilder::new()
            .language(language)
            .build()
            .map_err(|error| anyhow::anyhow!("{error}"))?;
        measurements.push(measure(iteration, || highlight(&source, formatter)));
    }

    let output_bytes = measurements
        .iter()
        .map(|measurement| measurement.output_bytes)
        .max()
        .unwrap_or_default();
    let deterministic = (measurements.len() > 1).then(|| {
        measurements
            .windows(2)
            .all(|pair| pair[0].output_sha256 == pair[1].output_sha256)
    });

    Ok(CaseResult {
        id: test_case.id.clone(),
        profile: test_case.profile.clone(),
        language: test_case.language.clone(),
        status: "ok",
        generated: test_case.generated.clone(),
        source_sha256: source_hash,
        origins: test_case.origins.clone(),
        deterministic,
        output_bytes,
        output_amplification: amplification(output_bytes, source.len()),
        iterations: measurements,
    })
}

fn measure<F>(iteration: usize, render: F) -> Iteration
where
    F: FnOnce() -> String,
{
    let before = rss_kb();
    let stop = Arc::new(AtomicBool::new(false));
    let peak = Arc::new(AtomicU64::new(before.unwrap_or_default()));
    let sampler_stop = Arc::clone(&stop);
    let sampler_peak = Arc::clone(&peak);
    let sampler = thread::spawn(move || sample_rss(&sampler_stop, &sampler_peak));
    let started = Instant::now();
    let output = render();
    let wall_ms = started.elapsed().as_millis();
    stop.store(true, Ordering::Relaxed);
    sampler.join().expect("RSS sampler must not panic");
    let after = rss_kb();
    let peak = optional_rss(peak.load(Ordering::Relaxed), before, after);

    Iteration {
        number: iteration,
        wall_ms,
        output_bytes: output.len(),
        output_sha256: sha256(output.as_bytes()),
        memory: Memory {
            before_rss: before,
            peak_rss: peak,
            after_rss: after,
            peak_delta: peak
                .zip(before)
                .map(|(peak, before)| peak.saturating_sub(before)),
        },
    }
}

fn sample_rss(stop: &AtomicBool, peak: &AtomicU64) {
    while !stop.load(Ordering::Relaxed) {
        if let Some(current) = rss_kb() {
            peak.fetch_max(current, Ordering::Relaxed);
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn optional_rss(sampled: u64, before: Option<u64>, after: Option<u64>) -> Option<u64> {
    if before.is_none() && after.is_none() {
        None
    } else {
        Some(
            sampled
                .max(before.unwrap_or_default())
                .max(after.unwrap_or_default()),
        )
    }
}

fn rss_kb() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        line.strip_prefix("VmRSS:")?
            .split_ascii_whitespace()
            .next()?
            .parse()
            .ok()
    })
}

fn violations(results: &[CaseResult], options: &Options) -> Vec<String> {
    let mut violations = Vec::new();
    for result in results {
        if result.deterministic == Some(false) {
            violations.push(format!("{}: output was nondeterministic", result.id));
        }
        for iteration in &result.iterations {
            if iteration.wall_ms > options.max_case_ms {
                violations.push(format!(
                    "{}: iteration {} took {} ms (budget {} ms)",
                    result.id, iteration.number, iteration.wall_ms, options.max_case_ms
                ));
            }
        }
        if result.output_amplification > options.max_output_amplification {
            violations.push(format!(
                "{}: output amplification {:.2}x (budget {}x)",
                result.id, result.output_amplification, options.max_output_amplification
            ));
        }
    }
    violations
}

#[allow(clippy::cast_precision_loss)]
fn amplification(output_bytes: usize, source_bytes: usize) -> f64 {
    output_bytes as f64 / source_bytes.max(1) as f64
}

fn sha256(bytes: &[u8]) -> String {
    lumis_wasm_runtime::sha256_hex(bytes)
}

fn write_report(path: &Path, report: &Report) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(report)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after Unix epoch")
        .as_millis()
}

fn git_revision() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
