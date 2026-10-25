//! The typedefs `tools/c-include/sys/types.h` and friends define.

use crate::{c_int, c_long, c_uint, c_ulong};

/// `size_t`
pub type size_t = usize;
/// `ssize_t` (a `long`, per `sys/types.h`).
pub type ssize_t = isize;
/// `off_t` (a `long`).
pub type off_t = i64;
/// `pid_t`
pub type pid_t = c_int;
/// `uid_t`
pub type uid_t = c_uint;
/// `gid_t`
pub type gid_t = c_uint;
/// `dev_t` — a 32-bit `(major << 16) | minor`, the port's device encoding.
pub type dev_t = c_uint;
/// `ino_t`
pub type ino_t = u64;
/// `mode_t`
pub type mode_t = c_uint;
/// `nlink_t`
pub type nlink_t = c_uint;
/// `blksize_t`
pub type blksize_t = c_long;
/// `blkcnt_t`
pub type blkcnt_t = c_long;
/// `fsblkcnt_t`
pub type fsblkcnt_t = u64;
/// `fsfilcnt_t`
pub type fsfilcnt_t = u64;
/// `time_t`
pub type time_t = i64;
/// `clockid_t`. The port's `time.h` spells this `long`, but `minix-libc`'s
/// `clock_gettime` takes a `c_int` and `rustix`'s `ClockId` is `repr(i32)`, so
/// the port's clock ids are 32-bit.
pub type clockid_t = c_int;
/// `nfds_t`
pub type nfds_t = c_ulong;
/// `intptr_t`
pub type intptr_t = isize;
/// `uintptr_t`
pub type uintptr_t = usize;
/// `intmax_t`
pub type intmax_t = i64;
/// `uintmax_t`
pub type uintmax_t = u64;

/// `major(dev)`: the device's major number, `dev >> 16`. The macros live in
/// `sys/types.h` (the BSD placement `libdrm`'s discovery path expects), so they
/// are `const fn`s here.
#[inline]
pub const fn major(dev: dev_t) -> c_uint {
    dev >> 16
}

/// `minor(dev)`: the low 16 bits of `dev`.
#[inline]
pub const fn minor(dev: dev_t) -> c_uint {
    dev & 0xffff
}

/// `makedev(ma, mi)`: the 32-bit encoding `major`/`minor` invert.
#[inline]
pub const fn makedev(ma: c_uint, mi: c_uint) -> dev_t {
    ((ma & 0xffff) << 16) | (mi & 0xffff)
}
