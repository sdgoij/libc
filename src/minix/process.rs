//! Process, environment, heap, signal and thread declarations (`unistd.h`,
//! `stdlib.h`, `signal.h`, `pthread.h`, `sys/wait.h`).
//!
//! The fd/path calls themselves live in `io.rs`; this module is the rest of what
//! `std` names. Signals minix does not deliver are still declared, at unique
//! numbers, so `cfg(unix)` consumers type-check.

use crate::prelude::*;
use crate::{c_char, c_int, c_long, c_uint, c_ulong, c_void, gid_t, pid_t, size_t, uid_t};

/// `EXIT_SUCCESS`
pub const EXIT_SUCCESS: c_int = 0;
/// `EXIT_FAILURE`
pub const EXIT_FAILURE: c_int = 1;

/// `STDIN_FILENO`
pub const STDIN_FILENO: c_int = 0;
/// `STDOUT_FILENO`
pub const STDOUT_FILENO: c_int = 1;
/// `STDERR_FILENO`
pub const STDERR_FILENO: c_int = 2;

/// `_SC_HOST_NAME_MAX`
pub const _SC_HOST_NAME_MAX: c_int = 180;

/// `FIOCLEX` — close-on-exec, in minix's NetBSD-style `_IO('f', 1)` encoding.
pub const FIOCLEX: c_ulong = 0x6601;

// ---- environment and heap ----

unsafe extern "C" {
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn getenv(name: *const c_char) -> *mut c_char;
    pub fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    pub fn unsetenv(name: *const c_char) -> c_int;
}

// ---- process ----

unsafe extern "C" {
    pub fn fork() -> pid_t;
    pub fn execvp(file: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn _exit(status: c_int) -> !;
    pub fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
    pub fn killpg(pgrp: pid_t, sig: c_int) -> c_int;
    pub fn sched_yield() -> c_int;
    pub fn setsid() -> pid_t;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int;
    pub fn setgroups(size: size_t, list: *const gid_t) -> c_int;
    pub fn getgroups(size: c_int, list: *mut gid_t) -> c_int;
    pub fn chdir(path: *const c_char) -> c_int;
    pub fn chroot(path: *const c_char) -> c_int;
    pub fn lchown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    pub fn mkfifo(path: *const c_char, mode: crate::mode_t) -> c_int;
    pub fn realpath(path: *const c_char, resolved: *mut c_char) -> *mut c_char;
    pub fn isatty(fd: c_int) -> c_int;
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
}

// ---- wait status ----

pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WCONTINUED: c_int = 8;

/// `WIFEXITED`
#[inline]
pub const fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

/// `WEXITSTATUS`
#[inline]
pub const fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

/// `WIFSIGNALED`
#[inline]
pub const fn WIFSIGNALED(status: c_int) -> bool {
    (status & 0x7f) != 0 && (status & 0x7f) != 0x7f
}

/// `WTERMSIG`
#[inline]
pub const fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

/// `WIFSTOPPED`
#[inline]
pub const fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

/// `WSTOPSIG`
#[inline]
pub const fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

/// `WCOREDUMP`
#[inline]
pub const fn WCOREDUMP(status: c_int) -> bool {
    (status & 0x80) != 0
}

/// `WIFCONTINUED` — PM has no stop/continue encoding, so a child is never
/// reported as having continued.
#[inline]
pub const fn WIFCONTINUED(_status: c_int) -> bool {
    false
}

// ---- signals ----

/// `sigset_t` (`signal.h`: `unsigned long[2]`)
pub type sigset_t = [c_ulong; 2];
/// `sighandler_t`
pub type sighandler_t = usize;
/// `SIG_DFL`
pub const SIG_DFL: sighandler_t = 0;
/// `SIG_IGN`
pub const SIG_IGN: sighandler_t = 1;
/// `SIG_ERR`
pub const SIG_ERR: sighandler_t = usize::MAX;

pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGBUS: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGUSR1: c_int = 10;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 12;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGCHLD: c_int = 20;
pub const SIGWINCH: c_int = 28;
pub const SIGSYS: c_int = 31;

// Minix does not dispatch these; the numbers fill the gaps its table leaves so
// that no two are equal.
pub const SIGSTKFLT: c_int = 16;
pub const SIGCONT: c_int = 17;
pub const SIGSTOP: c_int = 18;
pub const SIGTSTP: c_int = 19;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGIO: c_int = 29;
pub const SIGPWR: c_int = 30;

unsafe extern "C" {
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigismember(set: *const sigset_t, signum: c_int) -> c_int;
}

// ---- threads ----

/// `pthread_t`
pub type pthread_t = c_ulong;
/// `pthread_key_t`
pub type pthread_key_t = c_uint;
/// `PTHREAD_STACK_MIN`
pub const PTHREAD_STACK_MIN: size_t = 16 * 1024;

/// `pthread_attr_t`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pthread_attr_t {
    __x: c_int,
}

unsafe extern "C" {
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    pub fn pthread_detach(thread: pthread_t) -> c_int;
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, size: size_t) -> c_int;
    pub fn pthread_key_create(key: *mut pthread_key_t, destructor: *mut c_void) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    pub fn pthread_self() -> pthread_t;
}
