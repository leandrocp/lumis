use lumis_benchmarks::{showcase, validate_html};
use std::fs;

fn main() {
    let generated_dir = showcase::generated_dir();
    let assets_dir = generated_dir.join("assets");
    let documents = showcase::documents(&assets_dir);
    let syntaxes = showcase::syntect_syntaxes();

    for showcase_theme in &showcase::themes(&assets_dir) {
        let theme = showcase::lumis_theme(showcase_theme);
        let syntect_theme = showcase::syntect_theme(&assets_dir, showcase_theme);

        for document in &documents {
            let source =
                fs::read_to_string(assets_dir.join(&document.file)).expect("read showcase fixture");
            let fragments_dir = generated_dir
                .join("fragments")
                .join(&document.id)
                .join(&showcase_theme.id);
            fs::create_dir_all(&fragments_dir).expect("create fragments directory");

            let formatter = showcase::lumis_formatter(document, &theme);
            let lumis_output = showcase::highlight_lumis(&source, &formatter);
            validate_html(&lumis_output, source.len(), "Lumis Rust");
            fs::write(fragments_dir.join("lumis-rust.html"), lumis_output)
                .expect("write Lumis Rust fragment");

            if let Some(syntax) = showcase::syntect_syntax(&syntaxes, document) {
                let syntect_output =
                    showcase::highlight_syntect(&source, &syntaxes, syntax, &syntect_theme);
                validate_html(syntect_output.as_bytes(), source.len(), "syntect");
                fs::write(fragments_dir.join("syntect.html"), syntect_output)
                    .expect("write syntect fragment");
            }
        }
    }
}
