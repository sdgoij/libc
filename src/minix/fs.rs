//! Filesystem types and flags (`tools/c-include/fcntl.h`, `sys/stat.h`,
//! `sys/statvfs.h`, `dirent.h`).

use crate::prelude::*;
use crate::{
    c_char, c_int, c_long, c_short, c_uchar, c_ulong, c_void, dev_t, gid_t, ino_t, mode_t, nlink_t,
    off_t, size_t, uid_t,
};

/// `struct iovec` (`sys/uio.h`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

/// `struct stat` — the layout `minix-std`'s `Stat` and the VFS `fstat` reply use.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atime: i64,
    pub st_mtime: i64,
    pub st_ctime: i64,
}

/// `struct statfs` — the BSD/Linux form `rustix::fs::statfs` names. Minix
/// exposes `statvfs`; this exists so `rustix`'s `StatFs` alias type-checks.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statfs {
    pub f_type: c_long,
    pub f_bsize: c_long,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: fsid_t,
    pub f_namelen: c_long,
    pub f_frsize: c_long,
    pub f_flags: c_long,
    pub f_spare: [c_long; 4],
}

/// `struct statvfs` — the layout the FS server writes (32-bit `f_bsize`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct statvfs {
    pub f_flags: c_ulong,
    pub f_bsize: u32,
    pub f_frsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_favail: u64,
    pub f_fsid: c_ulong,
    pub f_flag: c_ulong,
    pub f_namemax: c_ulong,
}

/// `fsid_t`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fsid_t {
    pub __val: [c_int; 2],
}

/// `DIR` — opaque, as `dirent.h` declares it.
#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

/// `struct dirent`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct dirent {
    pub d_ino: ino_t,
    pub d_off: off_t,
    pub d_reclen: c_short,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

/// `DT_*` dirent types.
pub const DT_UNKNOWN: c_uchar = 0;
pub const DT_FIFO: c_uchar = 1;
pub const DT_CHR: c_uchar = 2;
pub const DT_DIR: c_uchar = 4;
pub const DT_BLK: c_uchar = 6;
pub const DT_REG: c_uchar = 8;
pub const DT_LNK: c_uchar = 10;
pub const DT_SOCK: c_uchar = 12;

/// `O_*` open flags. The values minix defines are the header's; the rest are
/// given non-colliding bits, since minix's `open` ignores them.
pub const O_RDONLY: c_int = 0o00;
pub const O_WRONLY: c_int = 0o01;
pub const O_RDWR: c_int = 0o02;
pub const O_ACCMODE: c_int = 0o03;
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;
pub const O_NOCTTY: c_int = 0o400;
pub const O_TRUNC: c_int = 0o1000;
pub const O_APPEND: c_int = 0o2000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_ASYNC: c_int = 0o20000;
pub const O_DIRECT: c_int = 0o40000;
pub const O_CLOEXEC: c_int = 0o200000;
pub const O_DIRECTORY: c_int = 0o400000;
pub const O_NOFOLLOW: c_int = 0o1000000;
pub const O_SYNC: c_int = 0o2000000;
pub const O_DSYNC: c_int = 0o4000000;

/// `F_*` fcntl commands (`fcntl.h`).
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;
/// Linux's `F_DUPFD_CLOEXEC`, which minix does not have.
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const FD_CLOEXEC: c_int = 1;

/// `F_*LCK` lock types (`struct flock.l_type`).
pub const F_RDLCK: c_short = 0;
pub const F_WRLCK: c_short = 1;
pub const F_UNLCK: c_short = 2;

/// `SEEK_*`
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

/// `access`/`faccessat` modes.
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;

/// The `*at` dirfd sentinel and flags. Minix has no `*at` semantics; these let
/// `rustix`'s `AtFlags` and `CWD` type-check.
pub const AT_FDCWD: c_int = -100;
pub const AT_EACCESS: c_int = 0x100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_REMOVEDIR: c_int = 0x800;

/// `flock(2)` operations.
pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

/// `fallocate` modes (Linux numbering; minix's `posix_fallocate` ignores them).
pub const FALLOC_FL_KEEP_SIZE: c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE: c_int = 0x02;
pub const FALLOC_FL_NO_HIDE_STALE: c_int = 0x04;
pub const FALLOC_FL_COLLAPSE_RANGE: c_int = 0x08;
pub const FALLOC_FL_ZERO_RANGE: c_int = 0x10;
pub const FALLOC_FL_INSERT_RANGE: c_int = 0x20;
pub const FALLOC_FL_UNSHARE_RANGE: c_int = 0x40;

/// `posix_fadvise` advice values.
pub const POSIX_FADV_NORMAL: c_int = 0;
pub const POSIX_FADV_RANDOM: c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: c_int = 2;
pub const POSIX_FADV_WILLNEED: c_int = 3;
pub const POSIX_FADV_DONTNEED: c_int = 4;
pub const POSIX_FADV_NOREUSE: c_int = 5;

/// `PIPE_BUF`
pub const PIPE_BUF: usize = 4096;

/// `utimensat` `tv_nsec` sentinels (Linux numbering; minix has no `utimensat`).
pub const UTIME_NOW: c_long = 0x3fff_ffff;
pub const UTIME_OMIT: c_long = 0x3fff_fffe;

/// `ST_*` `statvfs` mount flags.
pub const ST_RDONLY: c_ulong = 0x0001;
pub const ST_NOSUID: c_ulong = 0x0002;

/// `S_IF*` file types (`sys/stat.h`).
pub const S_IFMT: mode_t = 0o170000;
pub const S_IFSOCK: mode_t = 0o140000;
pub const S_IFLNK: mode_t = 0o120000;
pub const S_IFREG: mode_t = 0o100000;
pub const S_IFBLK: mode_t = 0o060000;
pub const S_IFDIR: mode_t = 0o040000;
pub const S_IFCHR: mode_t = 0o020000;
pub const S_IFIFO: mode_t = 0o010000;

/// `S_IS*` mode bits.
pub const S_ISUID: mode_t = 0o4000;
pub const S_ISGID: mode_t = 0o2000;
pub const S_ISVTX: mode_t = 0o1000;
pub const S_IRWXU: mode_t = 0o700;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;
pub const S_IRWXG: mode_t = 0o070;
pub const S_IRGRP: mode_t = 0o040;
pub const S_IWGRP: mode_t = 0o020;
pub const S_IXGRP: mode_t = 0o010;
pub const S_IRWXO: mode_t = 0o007;
pub const S_IROTH: mode_t = 0o004;
pub const S_IWOTH: mode_t = 0o002;
pub const S_IXOTH: mode_t = 0o001;

/// `struct flock`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct flock {
    pub l_type: c_short,
    pub l_whence: c_short,
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: c_int,
}
