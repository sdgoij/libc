//! `grp.h`: the group-entry lookups.
//!
//! The port reads `/etc/group`; the `_r` pair answers `ENOENT` when no entry
//! matches, as `tools/c-include/grp.h` documents. `gr_mem` is the
//! NULL-terminated pointer array POSIX requires, laid out after every string in
//! the caller's buffer.

use crate::prelude::*;
use crate::{c_char, c_int, gid_t, size_t};

/// `struct group`, in the order the header declares it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: gid_t,
    pub gr_mem: *mut *mut c_char,
}

unsafe extern "C" {
    pub fn getgrnam_r(
        name: *const c_char,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> c_int;
    pub fn getgrgid_r(
        gid: gid_t,
        grp: *mut group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut group,
    ) -> c_int;
    pub fn getgrnam(name: *const c_char) -> *mut group;
    pub fn getgrgid(gid: gid_t) -> *mut group;
}
