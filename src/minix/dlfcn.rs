//! `dlfcn.h` — the dynamic-loader surface `rustix`'s weak-symbol probe uses.

use crate::{c_char, c_void};

/// `RTLD_DEFAULT` — the global symbol scope.
pub const RTLD_DEFAULT: *mut c_void = core::ptr::null_mut();

unsafe extern "C" {
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}
