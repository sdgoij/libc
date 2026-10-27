//! Memory mapping (`tools/c-include/sys/mman.h`) and the names `rustix` and
//! `std` resolve from it.
//!
//! The port's VM does the work behind `mmap`/`munmap`/`mprotect`; `msync` and
//! `madvise` are accepted and do nothing there, and there is no pageable backing
//! store, so `mlock`/`munlock` have nothing to pin. The constants are the
//! header's, which follow Linux's numbering for the ones the port does not use.

use crate::{c_char, c_int, c_void, off_t, size_t};

pub const PROT_NONE: c_int = 0x00;
pub const PROT_READ: c_int = 0x01;
pub const PROT_WRITE: c_int = 0x02;
pub const PROT_EXEC: c_int = 0x04;

pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
/// `MAP_SHARED_VALIDATE` — the two low bits shared with the flags above.
pub const MAP_SHARED_VALIDATE: c_int = 0x03;
pub const MAP_FIXED: c_int = 0x10;
pub const MAP_ANONYMOUS: c_int = 0x20;
/// The BSD spelling of [`MAP_ANONYMOUS`], which `memmap2` and the crates that
/// follow it name.
pub const MAP_ANON: c_int = MAP_ANONYMOUS;
pub const MAP_GROWSDOWN: c_int = 0x0100;
pub const MAP_DENYWRITE: c_int = 0x0800;
pub const MAP_LOCKED: c_int = 0x2000;
pub const MAP_NORESERVE: c_int = 0x4000;
pub const MAP_POPULATE: c_int = 0x8000;
pub const MAP_STACK: c_int = 0x2_0000;
pub const MAP_HUGETLB: c_int = 0x4_0000;
pub const MAP_SYNC: c_int = 0x8_0000;
pub const MAP_FIXED_NOREPLACE: c_int = 0x10_0000;
/// `MAP_HUGE_2MB` / `MAP_HUGE_1GB` — the huge-page size selector, as on Linux.
pub const MAP_HUGE_2MB: c_int = 21 << 26;
/// `MAP_HUGE_1GB`
pub const MAP_HUGE_1GB: c_int = 30 << 26;

/// `MAP_FAILED`
pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;

pub const MS_ASYNC: c_int = 1;
pub const MS_INVALIDATE: c_int = 2;
pub const MS_SYNC: c_int = 4;

pub const POSIX_MADV_NORMAL: c_int = 0;
pub const POSIX_MADV_RANDOM: c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: c_int = 2;
pub const POSIX_MADV_WILLNEED: c_int = 3;
pub const POSIX_MADV_DONTNEED: c_int = 4;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;

unsafe extern "C" {
    pub fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, length: size_t, prot: c_int) -> c_int;
    pub fn msync(addr: *mut c_void, length: size_t, flags: c_int) -> c_int;
    pub fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    pub fn mlock(addr: *const c_void, length: size_t) -> c_int;
    pub fn munlock(addr: *const c_void, length: size_t) -> c_int;
    pub fn posix_madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    // Declared for the consumers that name them; the port answers ENOSYS (it has
    // no POSIX shared-memory namespace — the System V `shmget`/`shmat` interface
    // is what it implements).
    pub fn shm_open(name: *const c_char, oflag: c_int, mode: crate::mode_t) -> c_int;
    pub fn shm_unlink(name: *const c_char) -> c_int;
}
