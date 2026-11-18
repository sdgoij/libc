//! Terminal attributes and window size (`tools/c-include/termios.h`, `sys/ioctl.h`).
//!
//! The module is named `terminal` rather than `termios` because the C type of
//! the same name would collide with it in the type namespace.
//!
//! The layout below is the tty server's, which `tools/c-include/termios.h` and
//! `minix_std::termios` mirror. The NetBSD ioctl request numbers encode
//! `sizeof(struct termios)`, so the struct cannot change shape on its own.

use crate::prelude::*;
use crate::{c_int, c_uint, c_ulong, c_ushort};

/// A control character.
pub type cc_t = u8;
/// A baud rate. The port keeps the rate as a plain number in `c_ispeed` and
/// `c_ospeed` rather than as bits in `c_cflag`.
pub type speed_t = c_uint;
/// A terminal flag word.
pub type tcflag_t = c_uint;

/// Control characters in `c_cc`.
pub const NCCS: usize = 20;

/// `struct termios`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; NCCS],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

/// `struct winsize`, what `TIOCGWINSZ` returns.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

/// `TIOCGWINSZ` (`_IOR('t', 104, struct winsize)`), the request the tty server
/// answers for the console lines and the pty masters alike.
pub const TIOCGWINSZ: c_ulong = 0x4008_7468;
/// `TIOCSWINSZ` (`_IOW('t', 103, struct winsize)`).
pub const TIOCSWINSZ: c_ulong = 0x8008_7467;

/// `c_cc` subscripts.
pub const VEOF: usize = 0;
pub const VEOL: usize = 1;
pub const VEOL2: usize = 2;
pub const VERASE: usize = 3;
pub const VWERASE: usize = 4;
pub const VKILL: usize = 5;
pub const VREPRINT: usize = 6;
pub const VINTR: usize = 8;
pub const VQUIT: usize = 9;
pub const VSUSP: usize = 10;
pub const VDSUSP: usize = 11;
pub const VSTART: usize = 12;
pub const VSTOP: usize = 13;
pub const VLNEXT: usize = 14;
pub const VDISCARD: usize = 15;
pub const VMIN: usize = 16;
pub const VTIME: usize = 17;
pub const VSTATUS: usize = 18;

// `c_iflag`
pub const IGNBRK: tcflag_t = 0x0000_0001;
pub const BRKINT: tcflag_t = 0x0000_0002;
pub const IGNPAR: tcflag_t = 0x0000_0004;
pub const PARMRK: tcflag_t = 0x0000_0008;
pub const INPCK: tcflag_t = 0x0000_0010;
pub const ISTRIP: tcflag_t = 0x0000_0020;
pub const INLCR: tcflag_t = 0x0000_0040;
pub const IGNCR: tcflag_t = 0x0000_0080;
pub const ICRNL: tcflag_t = 0x0000_0100;
pub const IXON: tcflag_t = 0x0000_0200;
pub const IXOFF: tcflag_t = 0x0000_0400;
pub const IXANY: tcflag_t = 0x0000_0800;
pub const IMAXBEL: tcflag_t = 0x0000_2000;

// `c_oflag`
pub const OPOST: tcflag_t = 0x0000_0001;
pub const ONLCR: tcflag_t = 0x0000_0002;
pub const OXTABS: tcflag_t = 0x0000_0004;
pub const ONOEOT: tcflag_t = 0x0000_0008;
pub const OCRNL: tcflag_t = 0x0000_0010;
pub const ONOCR: tcflag_t = 0x0000_0020;
pub const ONLRET: tcflag_t = 0x0000_0040;

// `c_cflag`
pub const CIGNORE: tcflag_t = 0x0000_0001;
pub const CSIZE: tcflag_t = 0x0000_0300;
pub const CS5: tcflag_t = 0x0000_0000;
pub const CS6: tcflag_t = 0x0000_0100;
pub const CS7: tcflag_t = 0x0000_0200;
pub const CS8: tcflag_t = 0x0000_0300;
pub const CSTOPB: tcflag_t = 0x0000_0400;
pub const CREAD: tcflag_t = 0x0000_0800;
pub const PARENB: tcflag_t = 0x0000_1000;
pub const PARODD: tcflag_t = 0x0000_2000;
pub const HUPCL: tcflag_t = 0x0000_4000;
pub const CLOCAL: tcflag_t = 0x0000_8000;
pub const CRTSCTS: tcflag_t = 0x0001_0000;
pub const CDTRCTS: tcflag_t = 0x0002_0000;
pub const MDMBUF: tcflag_t = 0x0010_0000;

// `c_lflag`
pub const ECHOKE: tcflag_t = 0x0000_0001;
pub const ECHOE: tcflag_t = 0x0000_0002;
pub const ECHOK: tcflag_t = 0x0000_0004;
pub const ECHO: tcflag_t = 0x0000_0008;
pub const ECHONL: tcflag_t = 0x0000_0010;
pub const ECHOPRT: tcflag_t = 0x0000_0020;
pub const ECHOCTL: tcflag_t = 0x0000_0040;
pub const ISIG: tcflag_t = 0x0000_0080;
pub const ICANON: tcflag_t = 0x0000_0100;
pub const ALTWERASE: tcflag_t = 0x0000_0200;
pub const IEXTEN: tcflag_t = 0x0000_0400;
pub const EXTPROC: tcflag_t = 0x0000_0800;
pub const TOSTOP: tcflag_t = 0x0040_0000;
pub const FLUSHO: tcflag_t = 0x0080_0000;
pub const NOKERNINFO: tcflag_t = 0x0200_0000;
pub const PENDIN: tcflag_t = 0x2000_0000;
pub const NOFLSH: tcflag_t = 0x8000_0000;

/// `tcsetattr` actions.
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

/// `tcflush` queues.
pub const TCIFLUSH: c_int = 1;
pub const TCOFLUSH: c_int = 2;
pub const TCIOFLUSH: c_int = 3;

/// `tcflow` actions.
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;

unsafe extern "C" {
    pub fn tcgetattr(fd: c_int, t: *mut termios) -> c_int;
    pub fn tcsetattr(fd: c_int, actions: c_int, t: *const termios) -> c_int;
    pub fn tcdrain(fd: c_int) -> c_int;
    pub fn tcflush(fd: c_int, queue: c_int) -> c_int;
    pub fn tcflow(fd: c_int, action: c_int) -> c_int;
    pub fn tcgetpgrp(fd: c_int) -> c_int;
    pub fn tcsetpgrp(fd: c_int, pgrp: c_int) -> c_int;
    pub fn cfgetospeed(t: *const termios) -> c_int;
    pub fn cfgetispeed(t: *const termios) -> c_int;
    pub fn cfsetospeed(t: *mut termios, speed: c_int) -> c_int;
    pub fn cfsetispeed(t: *mut termios, speed: c_int) -> c_int;
    pub fn cfmakeraw(t: *mut termios);
}
