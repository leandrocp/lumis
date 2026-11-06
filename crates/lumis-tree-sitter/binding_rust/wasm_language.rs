use std::{
    error,
    ffi::{CStr, CString},
    fmt,
    mem::{self, MaybeUninit},
    os::raw::c_char,
};

pub use wasmtime_c_api::wasmtime;

use crate::{Language, LanguageError, Parser, ffi, ts_free};

// Force Cargo to include wasmtime-c-api as a dependency of this crate,
// even though it is only used by the C code.
#[expect(unused, reason = "forces Cargo to link wasmtime-c-api")]
fn use_wasmtime() {
    wasmtime_c_api::wasm_engine_new();
}

#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct wasm_engine_t {
    pub(crate) engine: wasmtime::Engine,
}

pub struct WasmStore(*mut ffi::TSWasmStore);

unsafe impl Send for WasmStore {}
unsafe impl Sync for WasmStore {}

#[derive(Debug, PartialEq, Eq)]
pub struct WasmError {
    pub kind: WasmErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WasmErrorKind {
    Parse,
    Compile,
    Instantiate,
    Other,
}

impl WasmStore {
    pub fn new(engine: &wasmtime::Engine) -> Result<Self, WasmError> {
        unsafe {
            let mut error = MaybeUninit::<ffi::TSWasmError>::uninit();
            let store = ffi::ts_wasm_store_new(
                std::ptr::from_ref::<wasmtime::Engine>(engine)
                    .cast_mut()
                    .cast(),
                error.as_mut_ptr(),
            );
            if store.is_null() {
                Err(WasmError::new(error.assume_init()))
            } else {
                Ok(Self(store))
            }
        }
    }

    pub fn load_language(&mut self, name: &str, bytes: &[u8]) -> Result<Language, WasmError> {
        let name = CString::new(name).unwrap();
        unsafe {
            let mut error = MaybeUninit::<ffi::TSWasmError>::uninit();
            let language = ffi::ts_wasm_store_load_language(
                self.0,
                name.as_ptr(),
                bytes.as_ptr().cast::<c_char>(),
                bytes.len() as u32,
                error.as_mut_ptr(),
            );
            if language.is_null() {
                Err(WasmError::new(error.assume_init()))
            } else {
                Ok(Language(language))
            }
        }
    }

    /// Load a language like [`WasmStore::load_language`], taking its compiled code
    /// from `precompiled`, a file holding what [`wasmtime::Engine::precompile_module`]
    /// produced for `bytes` with this store's engine.
    ///
    /// # Safety
    ///
    /// The engine rejects a file built by another Wasmtime version or configuration,
    /// but runs the code in one it accepts without further checks, as
    /// [`wasmtime::Module::deserialize_file`] does. The file must be trusted like the
    /// process's own code.
    pub unsafe fn load_language_precompiled(
        &mut self,
        name: &str,
        bytes: &[u8],
        precompiled: &std::path::Path,
    ) -> Result<Language, WasmError> {
        let name = CString::new(name).unwrap();
        let Some(path) = precompiled.to_str().and_then(|path| CString::new(path).ok()) else {
            return Err(WasmError {
                kind: WasmErrorKind::Compile,
                message: format!("unusable precompiled module path {}", precompiled.display()),
            });
        };
        unsafe {
            let mut error = MaybeUninit::<ffi::TSWasmError>::uninit();
            let language = ffi::ts_wasm_store_load_language_precompiled(
                self.0,
                name.as_ptr(),
                bytes.as_ptr().cast::<c_char>(),
                bytes.len() as u32,
                path.as_ptr(),
                error.as_mut_ptr(),
            );
            if language.is_null() {
                Err(WasmError::new(error.assume_init()))
            } else {
                Ok(Language(language))
            }
        }
    }

    #[must_use]
    pub fn language_count(&self) -> usize {
        unsafe { ffi::ts_wasm_store_language_count(self.0) }
    }
}

impl WasmError {
    unsafe fn new(error: ffi::TSWasmError) -> Self {
        let message = unsafe { CStr::from_ptr(error.message) }
            .to_str()
            .unwrap()
            .to_string();
        unsafe { ts_free(error.message.cast()) };
        Self {
            kind: match error.kind {
                ffi::TSWasmErrorKindParse => WasmErrorKind::Parse,
                ffi::TSWasmErrorKindCompile => WasmErrorKind::Compile,
                ffi::TSWasmErrorKindInstantiate => WasmErrorKind::Instantiate,
                _ => WasmErrorKind::Other,
            },
            message,
        }
    }
}

impl Language {
    #[must_use]
    pub fn is_wasm(&self) -> bool {
        unsafe { ffi::ts_language_is_wasm(self.0) }
    }
}

impl Parser {
    pub fn set_wasm_store(&mut self, store: WasmStore) -> Result<(), LanguageError> {
        unsafe { ffi::ts_parser_set_wasm_store(self.0.as_ptr(), store.0) };
        mem::forget(store);
        Ok(())
    }

    pub fn take_wasm_store(&mut self) -> Option<WasmStore> {
        let ptr = unsafe { ffi::ts_parser_take_wasm_store(self.0.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(WasmStore(ptr))
        }
    }
}

impl Drop for WasmStore {
    fn drop(&mut self) {
        unsafe { ffi::ts_wasm_store_delete(self.0) };
    }
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            WasmErrorKind::Parse => "Failed to parse Wasm",
            WasmErrorKind::Compile => "Failed to compile Wasm",
            WasmErrorKind::Instantiate => "Failed to instantiate Wasm module",
            WasmErrorKind::Other => "Unknown error",
        };
        write!(f, "{kind}: {}", self.message)
    }
}

impl error::Error for WasmError {}
