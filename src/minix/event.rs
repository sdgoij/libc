//! `poll` types and flags (`tools/c-include/poll.h`, `sys/ioctl.h`).

use crate::prelude::*;
use crate::{c_int, c_short, c_ulong};

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

/// `FIONBIO` — BSD `_IOW('f', 126, int)`, the encoding `sys/ioctl.h` builds.
pub const FIONBIO: c_ulong = 0x8004_667e;
/// `FIONREAD` — BSD `_IOR('f', 127, int)`.
pub const FIONREAD: c_ulong = 0x4004_667f;
