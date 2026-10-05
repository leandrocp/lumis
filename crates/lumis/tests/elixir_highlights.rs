#![cfg(feature = "lang-elixir")]

use lumis::{languages::Language, HtmlLinkedBuilder};

#[test]
fn defmodule_do_uses_the_definition_style_without_recoloring_other_blocks() {
    let source = "defmodule Outer do\n  defmodule Inner do\n  end\n  def project do\n    if true do\n      quote do\n        :ok\n      end\n    end\n  end\nend\n";
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Elixir)
        .build()
        .unwrap();
    let html = lumis::highlight(source, formatter);

    assert!(html.contains(
        r#"<span class="l-keyword-function">defmodule</span> <span class="l-module">Outer</span> <span class="l-keyword-function">do</span>"#
    ));
    assert!(html.contains(
        r#"<span class="l-keyword-function">defmodule</span> <span class="l-module">Inner</span> <span class="l-keyword-function">do</span>"#
    ));
    assert_eq!(
        html.matches(r#"<span class="l-keyword-function">do</span>"#)
            .count(),
        2
    );
    assert_eq!(
        html.matches(r#"<span class="l-keyword">do</span>"#).count(),
        3
    );
}
