use ansi_to_tui::IntoText;
use lumis::{languages::Language, themes, TerminalBuilder};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Paragraph},
    Frame,
};

const JS_SOURCE: &str = r"export function greet(name) {
  return `Hello, ${name}!`
}
";

const RUST_SOURCE: &str = r#"fn main() {
    println!("Hello, world!");
}
"#;

fn highlight(source: &str, language: Language) -> String {
    let theme = themes::get("dracula").unwrap();
    let formatter = TerminalBuilder::new()
        .language(language)
        .theme(Some(theme))
        .build()
        .unwrap();

    let mut output = Vec::new();
    lumis::write_highlight(&mut output, source, formatter).unwrap();
    String::from_utf8(output).unwrap()
}

fn map_color(color: ratatui_core::style::Color) -> Color {
    match color {
        ratatui_core::style::Color::Reset => Color::Reset,
        ratatui_core::style::Color::Black => Color::Black,
        ratatui_core::style::Color::Red => Color::Red,
        ratatui_core::style::Color::Green => Color::Green,
        ratatui_core::style::Color::Yellow => Color::Yellow,
        ratatui_core::style::Color::Blue => Color::Blue,
        ratatui_core::style::Color::Magenta => Color::Magenta,
        ratatui_core::style::Color::Cyan => Color::Cyan,
        ratatui_core::style::Color::Gray => Color::Gray,
        ratatui_core::style::Color::DarkGray => Color::DarkGray,
        ratatui_core::style::Color::LightRed => Color::LightRed,
        ratatui_core::style::Color::LightGreen => Color::LightGreen,
        ratatui_core::style::Color::LightYellow => Color::LightYellow,
        ratatui_core::style::Color::LightBlue => Color::LightBlue,
        ratatui_core::style::Color::LightMagenta => Color::LightMagenta,
        ratatui_core::style::Color::LightCyan => Color::LightCyan,
        ratatui_core::style::Color::White => Color::White,
        ratatui_core::style::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        ratatui_core::style::Color::Indexed(index) => Color::Indexed(index),
    }
}

fn map_modifier(modifier: ratatui_core::style::Modifier) -> Modifier {
    let mut mapped = Modifier::empty();

    if modifier.contains(ratatui_core::style::Modifier::BOLD) {
        mapped |= Modifier::BOLD;
    }
    if modifier.contains(ratatui_core::style::Modifier::DIM) {
        mapped |= Modifier::DIM;
    }
    if modifier.contains(ratatui_core::style::Modifier::ITALIC) {
        mapped |= Modifier::ITALIC;
    }
    if modifier.contains(ratatui_core::style::Modifier::UNDERLINED) {
        mapped |= Modifier::UNDERLINED;
    }
    if modifier.contains(ratatui_core::style::Modifier::SLOW_BLINK) {
        mapped |= Modifier::SLOW_BLINK;
    }
    if modifier.contains(ratatui_core::style::Modifier::RAPID_BLINK) {
        mapped |= Modifier::RAPID_BLINK;
    }
    if modifier.contains(ratatui_core::style::Modifier::REVERSED) {
        mapped |= Modifier::REVERSED;
    }
    if modifier.contains(ratatui_core::style::Modifier::HIDDEN) {
        mapped |= Modifier::HIDDEN;
    }
    if modifier.contains(ratatui_core::style::Modifier::CROSSED_OUT) {
        mapped |= Modifier::CROSSED_OUT;
    }

    mapped
}

fn map_style(style: ratatui_core::style::Style) -> Style {
    Style {
        fg: style.fg.map(map_color),
        bg: style.bg.map(map_color),
        add_modifier: map_modifier(style.add_modifier),
        sub_modifier: map_modifier(style.sub_modifier),
        ..Style::default()
    }
}

fn map_alignment(alignment: ratatui_core::layout::Alignment) -> Alignment {
    match alignment {
        ratatui_core::layout::Alignment::Left => Alignment::Left,
        ratatui_core::layout::Alignment::Center => Alignment::Center,
        ratatui_core::layout::Alignment::Right => Alignment::Right,
    }
}

fn map_text(text: ratatui_core::text::Text<'static>) -> Text<'static> {
    Text {
        alignment: text.alignment.map(map_alignment),
        style: map_style(text.style),
        lines: text
            .lines
            .into_iter()
            .map(|line| Line {
                style: map_style(line.style),
                alignment: line.alignment.map(map_alignment),
                spans: line
                    .spans
                    .into_iter()
                    .map(|span| Span {
                        style: map_style(span.style),
                        content: span.content,
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn ui(frame: &mut Frame<'_>) {
    let js_ansi = highlight(JS_SOURCE, Language::JavaScript);
    let rust_ansi = highlight(RUST_SOURCE, Language::Rust);
    let combined = format!("{js_ansi}\n{rust_ansi}");
    let text = map_text(combined.into_text().unwrap());
    let paragraph = Paragraph::new(text).block(Block::bordered().title("Ratatui + Lumis"));
    frame.render_widget(paragraph, frame.area());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(ui)?;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}
