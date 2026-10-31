//! `pwd.h`: the password-entry lookups.
//!
//! The port reads `/etc/passwd`; the `_r` pair answers `ENOENT` when no entry
//! matches, as `tools/c-include/pwd.h` documents.

use crate::prelude::*;
use crate::{c_char, c_int, gid_t, size_t, uid_t};

/// `struct passwd`, in the order the header declares it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
}

unsafe extern "C" {
    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getpwnam(name: *const c_char) -> *mut passwd;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
}
