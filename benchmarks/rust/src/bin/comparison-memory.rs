//! One fresh process per measurement of the comparison's memory pass, started
//! by `benchmarks/scripts/comparison-memory.mjs`. It renders one document with
//! one library, the way the timing pass does, and prints the process's peak
//! resident set size. With `baseline` it renders nothing.

use lumis_benchmarks::showcase;
use std::{env, fs, mem::MaybeUninit, time::Instant};

fn main() {
    let mut args = env::args().skip(1);
    let usage = "usage: comparison-memory <lumis-rust|syntect|baseline> <document> [fragment]";
    let implementation = args.next().expect(usage);
    let document_id = args.next().expect(usage);
    let fragment_path = args.next();

    let assets_dir = showcase::generated_dir().join("assets");
    let document = showcase::timed_documents(&assets_dir)
        .into_iter()
        .find(|document| document.id == document_id)
        .unwrap_or_else(|| panic!("no showcase document is named {document_id}"));
    // The flavour the timing pass renders, so the output is the one it timed.
    let showcase_theme = showcase::themes(&assets_dir)
        .into_iter()
        .next()
        .expect("a showcase theme");
    let source =
        fs::read_to_string(assets_dir.join(&document.file)).expect("read showcase fixture");

    // Setup is building the highlighter and its first highlight, so it covers
    // whatever a library defers to that first call.
    let started = Instant::now();
    let fragment = match implementation.as_str() {
        "baseline" => None,
        "lumis-rust" => {
            let theme = showcase::lumis_theme(&showcase_theme);
            let formatter = showcase::lumis_formatter(&document, &theme);
            Some(showcase::highlight_lumis(&source, &formatter))
        }
        "syntect" => {
            let theme = showcase::syntect_theme(&assets_dir, &showcase_theme);
            let syntaxes = showcase::syntect_syntaxes();
            let syntax = showcase::syntect_syntax(&syntaxes, &document)
                .unwrap_or_else(|| panic!("syntect cannot highlight {document_id}"));
            Some(showcase::highlight_syntect(&source, &syntaxes, syntax, &theme).into_bytes())
        }
        other => panic!("no Rust comparison library is named {other}"),
    };

    // Read before the fragment is written, so the peak is building the
    // highlighter and the one highlight, not this harness's file I/O.
    let setup_ns = started.elapsed().as_nanos();
    let peak_rss_bytes = peak_rss_bytes();
    if let Some(fragment) = fragment {
        let path = fragment_path.expect("a fragment path is required to check the output");
        fs::write(path, fragment).expect("write fragment");
    }
    println!("{{\"peakRssBytes\":{peak_rss_bytes},\"setupNs\":{setup_ns}}}");
}

fn peak_rss_bytes() -> u64 {
    let mut usage = MaybeUninit::<libc::rusage>::uninit();
    // SAFETY: `getrusage` fills the whole struct when it returns 0.
    let usage = unsafe {
        assert_eq!(
            libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()),
            0,
            "getrusage failed"
        );
        usage.assume_init()
    };
    let max_rss = u64::try_from(usage.ru_maxrss).expect("non-negative peak RSS");
    // macOS reports bytes and Linux kibibytes.
    if cfg!(target_os = "macos") {
        max_rss
    } else {
        max_rss * 1024
    }
}
