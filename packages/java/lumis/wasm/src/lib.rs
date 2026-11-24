use lumis::formatter::Formatter;
use lumis::languages::Language;
use lumis::themes::{self, Theme};
use lumis::{BBCodeScopedBuilder, HtmlInlineBuilder, HtmlLinkedBuilder, TerminalBuilder};
use std::ptr;

// Zero is left unused so an unset formatter is never mistaken for a valid one.
const TERMINAL: i32 = 1;
const HTML_INLINE: i32 = 2;
const HTML_LINKED: i32 = 3;
const BBCODE: i32 = 4;

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[no_mangle]
pub extern "C" fn wasm_malloc(size: usize) -> *mut u8 {
    unsafe { malloc(size) }
}

/// # Safety
///
/// `ptr` must come from [`wasm_malloc`] and must not have been freed already.
#[no_mangle]
pub unsafe extern "C" fn wasm_free(ptr: *mut u8) {
    unsafe { free(ptr) }
}

/// Takes ownership of the three argument buffers and returns `pointer << 32 | length`.
///
/// # Safety
///
/// Each pointer must come from [`wasm_malloc`] and hold its length in initialised bytes.
#[no_mangle]
pub unsafe extern "C" fn highlight(
    code_ptr: *mut u8,
    code_len: usize,
    theme_ptr: *mut u8,
    theme_len: usize,
    lang_ptr: *mut u8,
    lang_len: usize,
    formatter_type: i32,
) -> i64 {
    let code = unsafe { consume_string(code_ptr, code_len) };
    let theme_name = unsafe { consume_string(theme_ptr, theme_len) };
    let lang = unsafe { consume_string(lang_ptr, lang_len) };

    let theme = match themes::get(&theme_name) {
        Ok(theme) => theme,
        Err(error) => {
            return leak_string(&format!("error: theme `{theme_name}` not found: {error}"));
        }
    };
    let language = Language::guess(Some(&lang), "");
    let formatter = build_formatter(language, theme, formatter_type)
        .unwrap_or_else(|| panic!("error: invalid formatter type: {formatter_type}"));

    leak_string(&lumis::highlight(&code, formatter))
}

/// Language ids, one per line, as `pointer << 32 | length`.
#[no_mangle]
pub extern "C" fn available_languages() -> i64 {
    let ids: Vec<&str> = lumis::languages::available_languages()
        .iter()
        .map(|language| language.id)
        .collect();
    leak_string(&ids.join("\n"))
}

/// Theme names, one per line, as `pointer << 32 | length`.
#[no_mangle]
pub extern "C" fn available_themes() -> i64 {
    let names: Vec<&str> = themes::available_themes()
        .map(|theme| theme.name.as_str())
        .collect();
    leak_string(&names.join("\n"))
}

fn build_formatter(
    language: Language,
    theme: Theme,
    formatter_type: i32,
) -> Option<Box<dyn Formatter>> {
    let formatter: Box<dyn Formatter> = match formatter_type {
        TERMINAL => Box::new(
            TerminalBuilder::new()
                .language(language)
                .theme(Some(theme))
                .build()
                .expect("terminal formatter"),
        ),
        HTML_INLINE => Box::new(
            HtmlInlineBuilder::new()
                .language(language)
                .theme(Some(theme))
                .build()
                .expect("HTML inline formatter"),
        ),
        HTML_LINKED => Box::new(
            HtmlLinkedBuilder::new()
                .language(language)
                .build()
                .expect("HTML linked formatter"),
        ),
        BBCODE => Box::new(
            BBCodeScopedBuilder::new()
                .language(language)
                .build()
                .expect("BBCode formatter"),
        ),
        _ => return None,
    };
    Some(formatter)
}

fn leak_string(text: &str) -> i64 {
    let len = text.len();
    let ptr = wasm_malloc(len);
    assert!(!ptr.is_null(), "error: allocation failed");
    unsafe { ptr::copy_nonoverlapping(text.as_ptr(), ptr, len) };
    (((ptr as usize as u64) << 32) | len as u64).cast_signed()
}

unsafe fn consume_string(ptr: *mut u8, len: usize) -> String {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let text = std::str::from_utf8(bytes).expect("UTF-8").to_owned();
    unsafe { wasm_free(ptr) };
    text
}

// Wizer snapshots the module after this runs, so no grammar is initialised at load time.
#[cfg(target_family = "wasm")]
#[export_name = "wizer.initialize"]
pub extern "C" fn initialize() {
    let theme = themes::get("github_dark").expect("github_dark theme");
    for language in Language::iter() {
        for formatter_type in [TERMINAL, HTML_INLINE, HTML_LINKED, BBCODE] {
            if let Some(formatter) = build_formatter(language, theme.clone(), formatter_type) {
                lumis::highlight("", formatter);
            }
        }
    }
}
