---
title: Community
description: Community runtimes, projects using Lumis, and articles and talks about it.
---

## Community runtimes

The community maintains the Java and Python packages. Each has its own API and releases; check the package documentation for supported features.

| Runtime | Package | Maintainer | Guide |
| --- | --- | --- | --- |
| Java | [Lumis4J](https://github.com/roastedroot/lumis4j) | [roastedroot](https://github.com/roastedroot) | [Java](/usage/java) |
| Python | [fastpylight](https://github.com/AnswerDotAI/fastpylight) | [Answer.AI](https://github.com/AnswerDotAI) | [Python](/usage/python) |

See the [runtime reference](/reference/runtimes) for all seven runtimes.

For Kotlin on the JVM, see [kmp-play's Lumis4J example](https://github.com/sureshg/kmp-play/blob/main/jvm/app/src/App.kt).

## Used by

Each entry links to source code or the project's own write-up. Optional integrations are noted.

### Websites and publishing

| Project | How it uses Lumis | Integration |
| --- | --- | --- |
| [Hex.pm](https://hex.pm) | Highlights package source and individual code lines. | [Elixir source](https://github.com/hexpm/hexpm/blob/main/lib/hexpm_web/syntax_highlight.ex) |
| [Tuist](https://tuist.dev) | Configures MDEx to use Lumis for Markdown highlighting. | [Server configuration](https://github.com/tuist/tuist/blob/main/server/config/config.exs) |
| [Oban Pro](https://oban.pro) | Highlights code examples on its website through MDEx. | [Team's explanation](https://elixirforum.com/t/unlocking-agentic-workflows-with-oban/72570/) and [live article](https://oban.pro/articles/unlocking-agentic-workflows-with-oban-pro) |
| [Petal Components](https://petal.build) | Precompiles highlighted HEEx for component showcases through MDEx. | [Showcase renderer](https://github.com/petalframework/petal_components/blob/main/lib/petal_components/showcase.ex) |
| [jacko.io](https://jacko.io) | Highlights blog code blocks with Solarized light and dark themes. | [Rust renderer](https://github.com/oconnor663/jacko.io/blob/HEAD/render_posts/src/main.rs) |
| [Fiqry's website](https://fiqry.dev) | Uses a TypeScript rehype plugin to highlight MDX code blocks. | [Rehype plugin](https://github.com/fiqryq/site/blob/main/src/mdx/rehype-lumis.ts) |
| [ArchiDep](https://archidep.ch) | Highlights code in HEIG-VD's Architecture & Deployment course material. | [Course renderer](https://github.com/ArchiDep/website/blob/main/app/lib/archidep/course_site/renderer/highlighter.ex) |
| [Varsel](https://cna.erlef.org) | Renders highlighted code blocks for the Erlang Ecosystem Foundation's CVE Numbering Authority. | [Code-block renderer](https://github.com/erlef-cna/varsel/blob/main/lib/varsel/markdown/code_block.ex) |
| [ElixirStream](https://elixirstream.dev) | Highlights changelog code blocks through MDEx in its generator-diff tool. | [Changelog renderer](https://github.com/zestcreative/elixirstream/blob/main/lib/utility_web/controllers/gen_diff_html.ex) |
| [Aster](https://github.com/Wybxc/aster) | Uses Lumis's highlighting events and themes in a Typst-based static site generator. | [Rust highlighter](https://github.com/Wybxc/aster/blob/main/src/build/transform/highlight.rs) |
| [nixsearch](https://github.com/benkoppe/nixsearch) | Highlights Nix and other code blocks in documentation. | [Documentation renderer](https://github.com/benkoppe/nixsearch/blob/main/crates/web/src/render_docs.rs) |
| [lobo_tuerto](https://lobotuerto.com) | Highlights fenced code blocks in a Phoenix blog built around MDEx. | [Author's write-up](https://lobotuerto.com/blog/a-personal-site-is-never-finished) |
| [Runcom](https://runcom.org) | Highlights Elixir examples on its infrastructure-runbook website. | [Code component](https://github.com/tv-labs/runcom/blob/main/website/lib/website/code.ex) |
| [Sachith Shetty's website](https://shettysach.github.io) | Highlights Markdown code blocks in a custom Rust static site generator. | [Markdown renderer](https://github.com/shettysach/shettysach.github.io/blob/main/src/iter.rs) |
| [Zane Riley's website](https://zaneriley.com) | Uses Lumis tokens in its custom code-block renderer. | [Tokenizer](https://github.com/zaneriley/personal-site/blob/main/lib/portfolio/content/code/tokenizer/lumis.ex) |
| [Once](https://buildonce.dev) | Highlights documentation and blog code blocks with light and dark themes. | [Documentation renderer](https://github.com/tuist/once/blob/main/web/lib/once_site_web/docs/markdown.ex) |
| [Condukt](https://condukt.tuist.dev) | Highlights documentation and blog code blocks with light and dark themes. | [Documentation renderer](https://github.com/tuist/condukt/blob/main/web/lib/condukt_site_web/docs/markdown.ex) |
| [Plasma](https://plasma.dev) | Highlights documentation code blocks through MDEx. | [Documentation renderer](https://github.com/tuist/plasma/blob/main/web/lib/plasma_site_web/docs/markdown.ex) |
| [Gamend](https://gamend.org) | Highlights code in its guides, blog, and changelog with linked HTML through MDEx. | [Markdown renderer](https://github.com/appsinacup/gamend/blob/main/apps/gamend_core/lib/gamend/content/markdown.ex) |
| [MOSSLET](https://mosslet.com) | Highlights code in rendered Markdown through MDEx. | [Markdown renderer](https://github.com/moss-piglet/MOSSLET/blob/main/lib/mosslet/markdown_renderer.ex) and [engine configuration](https://github.com/moss-piglet/MOSSLET/blob/main/config/config.exs) |

### Python applications

These projects use the community-maintained [fastpylight runtime](/usage/python).

| Project | How it uses Lumis | Integration |
| --- | --- | --- |
| [mdhtml](https://github.com/AnswerDotAI/mdhtml) | Supports optional code highlighting for Markdown-to-HTML output through its `hl` extra. | [Python renderer](https://github.com/AnswerDotAI/mdhtml/blob/main/python/mdhtml/md2html.py) |
| [mdhtml2docx](https://github.com/AnswerDotAI/mdhtml2docx) | Supports optional code highlighting in Word documents using fastpylight tokens and themes. | [Token handling](https://github.com/AnswerDotAI/mdhtml2docx/blob/main/mdhtml2docx/hilite.py) |
| [fr-docs](https://github.com/Omena0/fr-docs) | Highlights fenced code and source panels in generated documentation. | [Python highlighter](https://github.com/Omena0/fr-docs/blob/main/fr_docs/syntax.py) |
| [Ramabana](https://github.com/vedicreader/ramabana) | Uses fastpylight tokens and themes to color code fences and diffs in its Python terminal interface. | [Terminal renderer](https://github.com/vedicreader/ramabana/blob/main/ramabana/cli.py) |

### Developer tools

| Project | How it uses Lumis | Integration |
| --- | --- | --- |
| [see](https://github.com/guilhermeprokisch/see) | Displays code and Markdown in the terminal with configurable Lumis themes. | [Usage and configuration](https://github.com/guilhermeprokisch/see#readme) |
| [termframe](https://github.com/pamburus/termframe) | Highlights terminal output for SVG screenshots. | [Rust highlighter](https://github.com/pamburus/termframe/blob/HEAD/src/syntax.rs) |
| [rs-rich-cli](https://github.com/buchochelliq-labs/rs-rich-cli) | Has a Lumis backend for its Rust implementation of Rich. Its Python bindings support Lumis when built with `--features lumis`. | [rich-lumis package](https://github.com/buchochelliq-labs/rs-rich-cli/tree/main/crates/rich-lumis) and [Python bindings](https://github.com/buchochelliq-labs/rs-rich-cli/tree/main/crates/rich-py) |
| [RPGMTranslate](https://github.com/RPG-Maker-Translation-Tools/rpgmtranslate-qt) | Highlights JSON, JavaScript, and Ruby in its Qt desktop app through a Rust backend. | [Rust highlighter](https://github.com/RPG-Maker-Translation-Tools/rpgmtranslate-qt/blob/main/rust/src/api.rs) and [Qt code viewer](https://github.com/RPG-Maker-Translation-Tools/rpgmtranslate-qt/blob/main/src/AssetMenu/CodeViewer.hpp) |
| [Cditor](https://github.com/JYChen-8866/Cditor) | Highlights editable code blocks in a Rust/GPUI rich-text editor, with background work scoped to the viewport. | [Editor highlighter](https://github.com/JYChen-8866/Cditor/blob/main/crates/cditor-editor-gpui/src/features/code/highlight.rs) |
| [Pointer](https://github.com/DolceTriade/pointer) | Highlights source files on the server in a code-search tool under development. | [File viewer](https://github.com/DolceTriade/pointer/blob/main/src/pages/file_viewer.rs) |
| [Vibe](https://github.com/elixir-vibe/vibe) | Highlights code in its coding-agent terminal interface, with custom themes. | [Terminal highlighter](https://github.com/elixir-vibe/vibe/blob/HEAD/lib/vibe/tui/syntax.ex) |
| [X-Trace](https://github.com/feng19/x_trace) | Uses the JavaScript markdown-it integration inside a Svelte string inspector. | [Svelte component](https://github.com/feng19/x_trace/blob/master/assets/svelte/components/string_inspect_dialog.svelte) |
| [Volt](https://github.com/elixir-volt/volt) | Optionally highlights source frames in its development-error overlay. | [Error renderer](https://github.com/elixir-volt/volt/blob/master/lib/volt/dev/error.ex) |
| [Sagents Live Debugger](https://github.com/sagents-ai/sagents_live_debugger) | Highlights code in its agent-debugging dashboard. | [Code component](https://github.com/sagents-ai/sagents_live_debugger/blob/main/lib/sagents_live_debugger/core_components.ex) |
| [Prima](https://github.com/plausible/prima) | Uses Lumis in the component library's demo code panels. | [Demo renderer](https://github.com/plausible/prima/blob/main/demo/lib/demo_web/components/code_example.ex) |
| [reactive-tui](https://github.com/eas4ai/reactive-tui) | Converts Lumis highlighting into styled text for its terminal widgets and Markdown renderer. | [Rust highlighter](https://github.com/eas4ai/reactive-tui/blob/main/src/syntax/highlighter.rs) |
| [Textbin](https://github.com/chaba-dev/textbin) | Highlights pastes in both its Phoenix web UI and Rust CLI. | [Web renderer](https://github.com/chaba-dev/textbin/blob/main/lib/textbin_web/live/ui/paste_live.ex) and [CLI renderer](https://github.com/chaba-dev/textbin/blob/main/crates/cli/src/base/show.rs) |
| [Grasp](https://github.com/gfrancischelli/grasp) | Highlights source code cards in its call-chain review interface. | [Code highlighter](https://github.com/gfrancischelli/grasp/blob/main/grasp/lib/grasp/highlight.ex) |
| [Tackle](https://github.com/Makesesama/tackle) | Highlights source lines in its agent harness's web interface. | [Code highlighter](https://github.com/Makesesama/tackle/blob/main/apps/tackle_web/lib/tackle_web/highlight.ex) |
| [Rail](https://github.com/Dish-Books/rail) | Highlights both sides of a repository diff and returns HTML for each line. | [Diff highlighter](https://github.com/Dish-Books/rail/blob/main/lib/rail/git/utils/highlight_lines.ex) |
| [Omni UI](https://github.com/aaronrussell/omni_ui) | Highlights code in its LiveView agent-chat interface. | [Code helper](https://github.com/aaronrussell/omni_ui/blob/main/lib/omni/ui/helpers.ex) |
| [SlopUI](https://github.com/mylanconnolly/slop_ui) | Highlights HEEx examples in the component library's development showcase. | [Showcase helper](https://github.com/mylanconnolly/slop_ui/blob/main/dev/slop_ui/sink/helpers.ex) |
| [Keen Markdown](https://github.com/Keenmate/keen-markdown) | Highlights code on the server in its Markdown renderer. | [Code renderer](https://github.com/Keenmate/keen-markdown/blob/prod/lib/keen_markdown/renderer.ex) |
| [Astral](https://github.com/elixir-volt/astral) | Supports optional code highlighting through MDEx for generated sites. | [Integration guide](https://github.com/elixir-volt/astral/blob/master/guides/features/content-and-data.md) |
| [Clarity](https://github.com/team-alembic/clarity) | Highlights Markdown code blocks in its introspection UI when MDEx's Lumis backend is enabled. | [Markdown component](https://github.com/team-alembic/clarity/blob/main/lib/clarity/components/markdown_component.ex) |
| [Phoenix Streamdown](https://github.com/dannote/phoenix_streamdown) | Highlights code blocks in streamed Markdown for Phoenix LiveView. | [Streaming renderer](https://github.com/dannote/phoenix_streamdown/blob/master/lib/phoenix_streamdown.ex) |
| [pi-elixir](https://github.com/elixir-vibe/pi-elixir) | Uses Lumis's scoped BBCode output to display highlighted code in its pi bridge. | [Syntax module](https://github.com/elixir-vibe/pi-elixir/blob/master/packages/bridge/lib/pi/syntax.ex) |
| [Tilde](https://github.com/elixir-vibe/tilde) | Highlights code as HTML and terminal output in an experimental Elixir agent console. | [Code renderer](https://github.com/elixir-vibe/tilde/blob/main/lib/tilde/renderer/syntax_highlight.ex) |
| [LiteSkill](https://github.com/liteskill-io/liteskill) | Highlights code blocks in its desktop chat interface through MDEx. | [Markdown renderer](https://github.com/liteskill-io/liteskill/blob/main/lib/liteskill_web/markdown.ex) |

### Parser package users

These projects use the `@lumis-sh/wasm-*` grammar packages directly for parsing, analysis, or highlighting.

| Project | How it uses the parser packages | Integration |
| --- | --- | --- |
| [SocratiCode](https://github.com/giancarloerra/SocratiCode) | Parses HEEx and EEx templates for codebase analysis and dependency graphs. | [Template parser](https://github.com/giancarloerra/SocratiCode/blob/main/src/services/elixir-templates.ts) |
| [lmgrep](https://github.com/Aetherall/lmgrep) | Loads Lumis WASM grammars for semantic code search. | [Language catalog](https://github.com/Aetherall/lmgrep/blob/main/src/infrastructure/treesitter/LanguageCatalog.ts) |
| [Glimpse](https://github.com/bstncartwright/glimpse) | Uses Lumis's C, HTML, JSON, and SQL parsers in a terminal diff viewer. | [Parser configuration](https://github.com/bstncartwright/glimpse/blob/develop/src/syntax-languages.ts) |
| [OpenCode Indexer](https://github.com/jbpraxxys/opencode-indexer) | Builds syntax-tree indexes for semantic code search in OpenCode. | [Indexing engine](https://github.com/jbpraxxys/opencode-indexer/blob/main/src/engine.ts) |
| [dsh-code-graph](https://github.com/simomat/dsh-code-graph) | Uses the Go, Java, and Rust grammar packages for structural code graphs. | [Parser loader](https://github.com/simomat/dsh-code-graph/blob/main/src/parse/loader.ts) |
| [Flutter Config Manager](https://github.com/Hamza-Shewa/flutter-services-permission-manager) | Uses Lumis's Dart parser to analyze Flutter source in a VS Code extension. | [Dart parser](https://github.com/Hamza-Shewa/flutter-services-permission-manager/blob/master/src/features/semantics/dart-source.ts) |
| [agent-as-repo](https://github.com/0x7067/agent-as-repo) | Uses prebuilt Kotlin and Swift parsers as a build fallback. | [Grammar build script](https://github.com/0x7067/agent-as-repo/blob/main/scripts/build-grammar-wasm.ts) |

### Earlier Autumnus integrations

These projects use or describe Autumnus, Lumis's former name. We haven't verified whether they've moved to current Lumis releases.

| Project | How it uses Autumnus | Integration |
| --- | --- | --- |
| [Woodpecker UI](https://github.com/StarArawn/woodpecker_ui) | Colors editable text in a Bevy UI framework by converting Autumnus output into styled text. | [Text widget](https://github.com/StarArawn/woodpecker_ui/blob/main/src/widgets/syntax_highlighting.rs) |
| [Adaptive CLI](https://github.com/adaptive-ml/adpt) | Highlights recipe JSON schemas in the Adaptive Platform CLI. | [CLI implementation](https://github.com/adaptive-ml/adpt/blob/main/src/main.rs) |
| [marshallku.com](https://marshallku.com) | Highlights code blocks in a Rust static site generator. | [Code renderer](https://github.com/marshallku/blog/blob/master/crates/ssg/src/syntax_highlighter.rs) |
| [Bog](https://github.com/j0lol/bog) | Highlights blog code blocks with inline HTML. | [Post renderer](https://github.com/j0lol/bog/blob/trunk/src/post/render.rs) |
| [SQL Race](https://github.com/akatsuki-no-kage/sql-race) | Uses Autumnus's SQL grammar and theme data in a terminal SQL editor. | [Text area](https://github.com/akatsuki-no-kage/sql-race/blob/main/src/component/textarea/mod.rs) and [editor](https://github.com/akatsuki-no-kage/sql-race/blob/main/src/component/editor.rs) |
| [Nacre](https://github.com/vectorian-rs/nacre) | Highlights Markdown code blocks in a local dashboard for the Beads issue tracker. | [Markdown renderer](https://github.com/vectorian-rs/nacre/blob/main/src/markdown.rs) |
| [Ievgen Pyrogov's website](https://ievgenpyrogov.com) | The author describes using Autumnus through MDEx and Tableau for blog code blocks. | [Author's write-up](https://ievgenpyrogov.com/this-website/) |

### Extensions

- [NSIS language support](https://github.com/idleberg/nsis-org/tree/main/packages/lumis) provides a community language package for the JavaScript runtime. Try the [live demo](https://idleberg.github.io/nsis-org/lumis/).

## Mentions

Some earlier coverage uses the project's former name, Autumnus.

### Articles and interviews

| Article or interview | Author or publication | What it covers |
| --- | --- | --- |
| [Lumis: Syntax Highlighter powered by Tree-sitter](https://blog.master.dev/lumis-syntax-highlighter-powered-by-tree-sitter/) | Chris Coyier, Master.dev · July 16, 2026 | A short introduction to Lumis, its runtimes, and server-side highlighting. |
| [Syntax highlighting in Java, without the pain](https://chicory.dev/blog/syntax-highlight/) | Andrea Peruffo, Chicory · February 2, 2026 | How Lumis4J uses WebAssembly to run on the JVM and Wizer to reduce startup work. |
| [Leandro Pereira on MDEx](https://www.youtube.com/watch?v=IyDNtqlClhU) | Elixir Mentor · March 2026 | An interview with Lumis's author about MDEx, Lumis, open source, and AI. |
| [A personal site is never finished](https://lobotuerto.com/blog/a-personal-site-is-never-finished) | lobo_tuerto · July 23, 2026 | How the author uses MDEx and Lumis to render Markdown in a Phoenix blog. |
| [Rendering math and highlighting code](https://shettysach.github.io/rendering-math-and-highlighting-code/index.html) | Sachith Shetty · June 5, 2025 | A Rust site's approach to code highlighting with CSS classes. The article names Lumis; its examples use the earlier Autumnus API. |
| [This website](https://ievgenpyrogov.com/this-website/) | Ievgen Pyrogov · July 21, 2025 | Building a blog with Tableau, MDEx, and Autumnus, including requests for Caddy and fish language support. |

### News and discussions

- [Thinking Elixir #290](https://podcast.thinkingelixir.com/290) · February 3, 2026: Covers the rename from Autumn to Lumis and its use in MDEx.
- [Thinking Elixir #309](https://podcast.thinkingelixir.com/309) · June 23, 2026: Covers Hex.pm's adoption of MDEx and Lumis.
- [Lumis on r/rust](https://www.reddit.com/r/rust/comments/1r92a1w/lumis_syntax_highlighter_powered_by_treesitter/) · February 19, 2026: The author's introduction, followed by discussion of language detection, parser packaging, and binary size.
- [This Week in Bevy (Autumnus mention)](https://thisweekinbevy.com/issue/2025-05-26-the-bevy-cli-game-jam-prep-and-more) · May 26, 2025: The "Woodpecker UI Multi-line Text Editor" section mentions syntax highlighting with Autumnus, Lumis's former name.
- [Oban's highlighting setup](https://elixirforum.com/t/unlocking-agentic-workflows-with-oban/72570/) · September 2025: The Oban team explains its use of MDEx and Autumnus in an Elixir Forum discussion.
- [Syntax highlighting discussion on Lobsters](https://lobste.rs/s/mk6n0l/syntax_highlighting_with_tree_sitter#c_pwyuph) · March 30, 2025: A commenter describes using the Autumn Elixir wrapper and discusses CSS classes and themes.

## Add to this page

To add a project, runtime, or mention, [send a PR](https://github.com/leandrocp/lumis/blob/main/docs/content/community.md) with a short description and a link. For projects using Lumis, link to the integration code or a write-up.
