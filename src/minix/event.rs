//! `poll` types and flags (`tools/c-include/poll.h`, `sys/ioctl.h`).

use crate::prelude::*;
use crate::{c_int, c_short, c_uint, c_ulong, O_CLOEXEC};

/// `struct pollfd`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

pub const POLLIN: c_short = 0x001;
pub const POLLPRI: c_short = 0x002;
pub const POLLOUT: c_short = 0x004;
pub const POLLERR: c_short = 0x008;
pub const POLLHUP: c_short = 0x010;
pub const POLLNVAL: c_short = 0x020;
/// The `POLLRDNORM`..`POLLWRBAND` set is Linux's; minix's `poll(2)` never sets
/// these bits, but `rustix`'s `PollFlags` names them.
pub const POLLRDNORM: c_short = 0x040;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;

/// `FIONBIO` â€” BSD `_IOW('f', 126, int)`, the encoding `sys/ioctl.h` builds.
pub const FIONBIO: c_ulong = 0x8004_667e;
/// `FIONREAD` â€” BSD `_IOR('f', 127, int)`.
pub const FIONREAD: c_ulong = 0x4004_667f;

// ---- epoll(7) (`sys/epoll.h`) ----

/// `epoll_create1` flag: `O_CLOEXEC`.
pub const EPOLL_CLOEXEC: c_int = O_CLOEXEC;

/// `epoll_ctl` operations.
pub const EPOLL_CTL_ADD: c_int = 1;
pub const EPOLL_CTL_DEL: c_int = 2;
pub const EPOLL_CTL_MOD: c_int = 3;

/// `struct epoll_event.events` bits. The low bits are `poll(2)`'s; the rest are
/// the Linux-only names `rustix`'s `EventFlags` carries, which the port never
/// sets.
pub const EPOLLIN: c_uint = 0x001;
pub const EPOLLPRI: c_uint = 0x002;
pub const EPOLLOUT: c_uint = 0x004;
pub const EPOLLERR: c_uint = 0x008;
pub const EPOLLHUP: c_uint = 0x010;
pub const EPOLLNVAL: c_uint = 0x020;
pub const EPOLLRDNORM: c_uint = 0x040;
pub const EPOLLRDBAND: c_uint = 0x080;
pub const EPOLLWRNORM: c_uint = 0x100;
pub const EPOLLWRBAND: c_uint = 0x200;
pub const EPOLLMSG: c_uint = 0x400;
pub const EPOLLRDHUP: c_uint = 0x2000;
pub const EPOLLEXCLUSIVE: c_uint = 1 << 28;
pub const EPOLLWAKEUP: c_uint = 1 << 29;
/// One-shot and edge-triggered are accepted but served level-triggered.
pub const EPOLLONESHOT: c_uint = 1 << 30;
pub const EPOLLET: c_uint = 1 << 31;

/// `struct epoll_event` -- the kernel ABI packs `events` and `data`, so this is
/// 12 bytes, not 16.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct epoll_event {
    pub events: c_uint,
    pub data: u64,
}

unsafe extern "C" {
    pub fn epoll_create(size: c_int) -> c_int;
    pub fn epoll_create1(flags: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> c_int;
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;
}
