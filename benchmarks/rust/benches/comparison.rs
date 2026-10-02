//! Times Lumis Rust and syntect on every document of the visual comparison,
//! through the calls that rendered the published output.

use criterion::{criterion_group, criterion_main, Criterion};
use lumis_benchmarks::{measurement_time, sample_size, showcase, warm_up_time};
use std::env;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;

fn comparison(c: &mut Criterion) {
    let assets_dir = showcase::generated_dir().join("assets");
    let fragments_dir =
        PathBuf::from(env::var_os("BENCH_FRAGMENTS_DIR").expect("BENCH_FRAGMENTS_DIR"));
    // A flavour changes the colours a library writes, not how much work it does
    // to find them, so one flavour is timed: the first, as in the JavaScript run.
    let showcase_theme = showcase::themes(&assets_dir)
        .into_iter()
        .next()
        .expect("a showcase theme");
    let theme = showcase::lumis_theme(&showcase_theme);
    let syntect_theme = showcase::syntect_theme(&assets_dir, &showcase_theme);
    let syntaxes = showcase::syntect_syntaxes();

    for document in showcase::documents(&assets_dir) {
        let source =
            fs::read_to_string(assets_dir.join(&document.file)).expect("read showcase fixture");
        // Building the formatter and loading the syntaxes is setup, as building a
        // highlighter is for the JavaScript libraries, so neither is timed.
        let formatter = showcase::lumis_formatter(&document, &theme);
        let syntax = showcase::syntect_syntax(&syntaxes, &document);

        // What is timed is kept, so the timings can be checked against the
        // published pages and tied to the exact output they describe.
        let document_dir = fragments_dir.join(&document.id);
        fs::create_dir_all(&document_dir).expect("create fragments directory");
        fs::write(
            document_dir.join("lumis-rust.html"),
            showcase::highlight_lumis(&source, &formatter),
        )
        .expect("write Lumis Rust fragment");
        if let Some(syntax) = syntax {
            fs::write(
                document_dir.join("syntect.html"),
                showcase::highlight_syntect(&source, &syntaxes, syntax, &syntect_theme),
            )
            .expect("write syntect fragment");
        }

        let mut group = c.benchmark_group(&document.id);
        group.sample_size(sample_size());
        group.warm_up_time(warm_up_time());
        group.measurement_time(measurement_time());
        group.bench_function("lumis-rust", |b| {
            b.iter_with_large_drop(|| showcase::highlight_lumis(black_box(&source), &formatter));
        });
        if let Some(syntax) = syntax {
            group.bench_function("syntect", |b| {
                b.iter_with_large_drop(|| {
                    showcase::highlight_syntect(
                        black_box(&source),
                        &syntaxes,
                        syntax,
                        &syntect_theme,
                    )
                });
            });
        }
        group.finish();
    }
}

criterion_group!(benches, comparison);
criterion_main!(benches);
