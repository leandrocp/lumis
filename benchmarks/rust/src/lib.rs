//! Code the benchmarks share. The scenario suite and the visual comparison are
//! timed with the same settings, and the comparison's renderer and its timings
//! make the same calls, so the time on the page is the time of the output
//! beside it.

use std::{env, time::Duration};

pub mod showcase;

/// Criterion's sample count, from the variable the JavaScript runners read too.
/// Criterion refuses fewer than ten.
pub fn sample_size() -> usize {
    env::var("BENCH_SAMPLES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(20)
        .max(10)
}

pub fn warm_up_time() -> Duration {
    duration("BENCH_WARMUP_SECONDS", 0.5)
}

pub fn measurement_time() -> Duration {
    duration("BENCH_TIME_SECONDS", 1.0)
}

fn duration(name: &str, default: f64) -> Duration {
    Duration::from_secs_f64(
        env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default),
    )
}

pub fn validate_html(output: &[u8], input_bytes: usize, implementation: &str) {
    let html = std::str::from_utf8(output).expect("highlight output is UTF-8");
    assert!(
        output.len() > input_bytes && html.contains("<pre") && html.contains("<span"),
        "{implementation} did not produce highlighted HTML"
    );
}
