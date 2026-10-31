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

/// `_SC_CLK_TCK`
pub const _SC_CLK_TCK: c_int = 2;
/// `_SC_GETPW_R_SIZE_MAX`
pub const _SC_GETPW_R_SIZE_MAX: c_int = 69;
/// `_SC_PAGESIZE`
pub const _SC_PAGESIZE: c_int = 30;
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
    pub fn abort() -> !;
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
    pub fn fchdir(fd: c_int) -> c_int;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn umask(mask: crate::mode_t) -> crate::mode_t;
    pub fn getsid(pid: pid_t) -> pid_t;
    pub fn getpgrp() -> pid_t;
    pub fn getpgid(pid: pid_t) -> pid_t;
    pub fn nice(inc: c_int) -> c_int;
    pub fn chroot(path: *const c_char) -> c_int;
    pub fn lchown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    pub fn mkfifo(path: *const c_char, mode: crate::mode_t) -> c_int;
    pub fn realpath(path: *const c_char, resolved: *mut c_char) -> *mut c_char;
    pub fn isatty(fd: c_int) -> c_int;
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    pub fn strlen(s: *const c_char) -> size_t;
    /// POSIX 2008 form: 0 on success, `ERANGE` when the message was truncated.
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
}

// ---- sys/resource.h ----

/// `rlim_t`
pub type rlim_t = c_ulong;

/// `RLIM_INFINITY`
pub const RLIM_INFINITY: rlim_t = rlim_t::MAX;

/// `RLIMIT_CPU`
pub const RLIMIT_CPU: c_int = 0;
/// `RLIMIT_FSIZE`
pub const RLIMIT_FSIZE: c_int = 1;
/// `RLIMIT_DATA`
pub const RLIMIT_DATA: c_int = 2;
/// `RLIMIT_STACK`
pub const RLIMIT_STACK: c_int = 3;
/// `RLIMIT_CORE`
pub const RLIMIT_CORE: c_int = 4;
/// `RLIMIT_RSS`
pub const RLIMIT_RSS: c_int = 5;
/// `RLIMIT_NPROC`
pub const RLIMIT_NPROC: c_int = 6;
/// `RLIMIT_NOFILE`
pub const RLIMIT_NOFILE: c_int = 7;
/// `RLIMIT_MEMLOCK`
pub const RLIMIT_MEMLOCK: c_int = 8;
/// `RLIMIT_AS`
pub const RLIMIT_AS: c_int = 9;
/// `RLIMIT_LOCKS`
pub const RLIMIT_LOCKS: c_int = 10;
/// `RLIMIT_SIGPENDING`
pub const RLIMIT_SIGPENDING: c_int = 11;
/// `RLIMIT_MSGQUEUE`
pub const RLIMIT_MSGQUEUE: c_int = 12;
/// `RLIMIT_NICE`
pub const RLIMIT_NICE: c_int = 13;
/// `RLIMIT_RTPRIO`
pub const RLIMIT_RTPRIO: c_int = 14;
/// `RLIMIT_RTTIME`
pub const RLIMIT_RTTIME: c_int = 15;

/// `struct rlimit`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

/// `PRIO_PROCESS`
pub const PRIO_PROCESS: c_int = 0;
/// `PRIO_PGRP`
pub const PRIO_PGRP: c_int = 1;
/// `PRIO_USER`
pub const PRIO_USER: c_int = 2;

unsafe extern "C" {
    pub fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    pub fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    pub fn getpriority(which: c_int, who: c_int) -> c_int;
    pub fn setpriority(which: c_int, who: c_int, prio: c_int) -> c_int;
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

// ---- waitid ----

/// `WEXITED`
pub const WEXITED: c_int = 0x04;
/// `WSTOPPED`
pub const WSTOPPED: c_int = 0x02;
/// `WNOWAIT`
pub const WNOWAIT: c_int = 0x0100_0000;

/// `P_ALL`
pub const P_ALL: c_int = 0;
/// `P_PID`
pub const P_PID: c_int = 1;
/// `P_PGID`
pub const P_PGID: c_int = 2;

/// `CLD_EXITED`
pub const CLD_EXITED: c_int = 1;
/// `CLD_KILLED`
pub const CLD_KILLED: c_int = 2;
/// `CLD_DUMPED`
pub const CLD_DUMPED: c_int = 3;
/// `CLD_TRAPPED`
pub const CLD_TRAPPED: c_int = 4;
/// `CLD_STOPPED`
pub const CLD_STOPPED: c_int = 5;
/// `CLD_CONTINUED`
pub const CLD_CONTINUED: c_int = 6;

/// `siginfo_t` — the `waitid` result and a signal handler's `info`.
///
/// The header's trailing union is 8-byte aligned because `si_addr` is a
/// pointer, so it starts at offset 16; the SIGCHLD members `waitid` fills
/// overlap the same bytes, and `si_status` (offset 24) follows both.
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    _union_align: c_int,
    /// The union's pointer member: the faulting address a signal handler sees.
    pub si_addr: *mut c_void,
    /// The union's `si_status` member: the exit code, or the signal that killed
    /// the child.
    pub si_status: c_int,
    _union_tail: c_int,
}

impl siginfo_t {
    /// `si_status` — for `waitid`, the exit code, or the signal that killed the
    /// child.
    ///
    /// # Safety
    ///
    /// POSIX makes `si_status` meaningful only in a `siginfo_t` that a
    /// successful `waitid` filled.
    pub unsafe fn si_status(&self) -> c_int {
        self.si_status
    }
}

unsafe extern "C" {
    pub fn waitid(idtype: c_int, id: c_int, infop: *mut siginfo_t, options: c_int) -> c_int;
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

/// Signal disposition flags. The kernel does not interpret them yet.
pub const SA_NOCLDSTOP: c_int = 0x0000_0001;
pub const SA_NOCLDWAIT: c_int = 0x0000_0002;
pub const SA_SIGINFO: c_int = 0x0000_0004;
pub const SA_ONSTACK: c_int = 0x0800_0000;
pub const SA_RESTART: c_int = 0x1000_0000;
pub const SA_NODEFER: c_int = 0x4000_0000;
pub const SA_RESETHAND: c_int = 0x8000_0000u32 as c_int;

/// The `sigprocmask` operations.
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;

/// `struct sigaction` (`signal.h`): the handler address, the mask, the flags,
/// then the restorer. `minix-libc` reads the first 28 bytes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sigaction {
    pub sa_sigaction: sighandler_t,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: usize,
}

unsafe extern "C" {
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigfillset(set: *mut sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigismember(set: *const sigset_t, signum: c_int) -> c_int;
    pub fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    /// POSIX `signal()`: the BSD form of `sigaction` for one handler.
    pub fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    pub fn raise(sig: c_int) -> c_int;
    pub fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    /// POSIX's thread-aware spelling of `sigprocmask`; the port masks per
    /// process, so the two are the same call.
    pub fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    pub fn sigsuspend(mask: *const sigset_t) -> c_int;
    pub fn sigpending(set: *mut sigset_t) -> c_int;
    pub fn strsignal(sig: c_int) -> *mut c_char;
}

// ---- locale.h ----

/// `LC_CTYPE`
pub const LC_CTYPE: c_int = 0;
/// `LC_NUMERIC`
pub const LC_NUMERIC: c_int = 1;
/// `LC_TIME`
pub const LC_TIME: c_int = 2;
/// `LC_COLLATE`
pub const LC_COLLATE: c_int = 3;
/// `LC_MONETARY`
pub const LC_MONETARY: c_int = 4;
/// `LC_MESSAGES`
pub const LC_MESSAGES: c_int = 5;
/// `LC_ALL`
pub const LC_ALL: c_int = 6;

unsafe extern "C" {
    /// POSIX `setlocale()`: the port serves the C locale only, so this reports
    /// the current name and installs nothing.
    pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
}

// ---- sys/utsname.h ----

/// `struct utsname`: six 65-byte fields. The port's `sys/utsname.h` carries a
/// `domainname` where the BSDs do not, and PM serves the first five.
#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

unsafe extern "C" {
    pub fn uname(buf: *mut utsname) -> c_int;
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
    /// POSIX `pthread_atfork()`: the handlers `fork()` runs before and after the
    /// split, which is what `rand`'s fork counter registers.
    pub fn pthread_atfork(
        prepare: Option<unsafe extern "C" fn()>,
        parent: Option<unsafe extern "C" fn()>,
        child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
}
