//! The `extern "C"` declarations. The implementations are `crates/minix-libc`;
//! these must agree with it (and with `tools/c-include/`), which is what
//! `tools/check-c-headers.py` polices for the C side.
//!
//! `open`, `fcntl` and `ioctl` are variadic here even where `minix-libc`
//! declares fixed arguments, because `rustix` calls them with different arity
//! (`open` with 3, `fcntl` with 2 or 3); a variadic prototype accepts both and
//! the extra register is ignored by the callee.

use crate::{
    c_char, c_int, c_long, c_ulong, c_void, dev_t, dirent, gid_t, iovec, mode_t, nfds_t, off_t,
    pid_t, pollfd, size_t, ssize_t, timespec, uid_t, DIR,
};

/// `TIOCSCTTY` — become the controlling terminal (`_IO('t', 37)`, NetBSD's
/// number, which the port's ioctl encoding preserves). The tty server does not
/// act on it yet; the constant exists for the consumers that name the request.
pub const TIOCSCTTY: c_ulong = 0x7425;

unsafe extern "C" {
    pub fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    pub fn openat(dirfd: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn preadv(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn pwritev(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;
    pub fn fsync(fd: c_int) -> c_int;
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn sync();
    pub fn dup(fd: c_int) -> c_int;
    pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;
    pub fn pipe(fds: *mut c_int) -> c_int;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    pub fn pause() -> c_int;
    pub fn flock(fd: c_int, operation: c_int) -> c_int;
    pub fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advice: c_int) -> c_int;
    pub fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int;

    pub fn access(path: *const c_char, mode: c_int) -> c_int;
    pub fn faccessat(dirfd: c_int, path: *const c_char, mode: c_int, flags: c_int) -> c_int;
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fchmod(fd: c_int, mode: mode_t) -> c_int;
    pub fn fchmodat(dirfd: c_int, path: *const c_char, mode: mode_t, flags: c_int) -> c_int;
    pub fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    pub fn fchown(fd: c_int, owner: uid_t, group: gid_t) -> c_int;
    pub fn fchownat(
        dirfd: c_int,
        path: *const c_char,
        owner: uid_t,
        group: gid_t,
        flags: c_int,
    ) -> c_int;
    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    pub fn mkdirat(dirfd: c_int, path: *const c_char, mode: mode_t) -> c_int;
    pub fn mknodat(dirfd: c_int, path: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn unlink(path: *const c_char) -> c_int;
    pub fn unlinkat(dirfd: c_int, path: *const c_char, flags: c_int) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;
    pub fn renameat(
        olddirfd: c_int,
        old: *const c_char,
        newdirfd: c_int,
        new: *const c_char,
    ) -> c_int;
    pub fn link(old: *const c_char, new: *const c_char) -> c_int;
    pub fn linkat(
        olddirfd: c_int,
        old: *const c_char,
        newdirfd: c_int,
        new: *const c_char,
        flags: c_int,
    ) -> c_int;
    pub fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;
    pub fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int;
    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    pub fn readlinkat(
        dirfd: c_int,
        path: *const c_char,
        buf: *mut c_char,
        bufsiz: size_t,
    ) -> ssize_t;
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const timespec,
        flags: c_int,
    ) -> c_int;
    pub fn futimens(fd: c_int, times: *const timespec) -> c_int;

    pub fn stat(path: *const c_char, buf: *mut crate::stat) -> c_int;
    pub fn fstat(fd: c_int, buf: *mut crate::stat) -> c_int;
    pub fn lstat(path: *const c_char, buf: *mut crate::stat) -> c_int;
    pub fn fstatat(dirfd: c_int, path: *const c_char, buf: *mut crate::stat, flags: c_int)
        -> c_int;
    pub fn statfs(path: *const c_char, buf: *mut crate::statfs) -> c_int;
    pub fn fstatfs(fd: c_int, buf: *mut crate::statfs) -> c_int;
    pub fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int;
    pub fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int;

    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn fdopendir(fd: c_int) -> *mut DIR;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn closedir(dirp: *mut DIR) -> c_int;
    pub fn rewinddir(dirp: *mut DIR);
    pub fn seekdir(dirp: *mut DIR, loc: c_long);
    pub fn dirfd(dirp: *mut DIR) -> c_int;

    pub fn getuid() -> uid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getegid() -> gid_t;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
}
