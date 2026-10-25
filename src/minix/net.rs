//! Sockets (`tools/c-include/sys/socket.h`) and the names `std` resolves from
//! them. The net server implements the AF_UNIX and AF_INET families;
//! `setsockopt` is a no-op in the port, so its option values are placeholders.

use crate::prelude::*;
use crate::{c_char, c_int, c_void, size_t, ssize_t};

/// `socklen_t`
pub type socklen_t = u32;
/// `sa_family_t`
pub type sa_family_t = u16;

/// `struct sockaddr`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}

/// `struct sockaddr_storage` — big enough for any family.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_pad: [u8; 126],
}

/// `struct sockaddr_un`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [c_char; 108],
}

/// `struct in_addr`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr {
    pub s_addr: crate::c_uint,
}

/// `struct linger`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

pub const AF_UNIX: c_int = 1;
pub const AF_INET: c_int = 2;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;

/// Minix's `SOL_SOCKET` is `0xffff` (`sys/sys/socket.h`), not Linux's 1.
pub const SOL_SOCKET: c_int = 0xffff;
pub const SO_PEERCRED: c_int = 17;
// `setsockopt` is unsupported in the port; these exist for `std` to name.
pub const SO_ERROR: c_int = 4;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_LINGER: c_int = 13;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;

pub const MSG_OOB: c_int = 0x0001;
pub const MSG_PEEK: c_int = 0x0002;
pub const MSG_DONTROUTE: c_int = 0x0004;
pub const MSG_EOR: c_int = 0x0008;
pub const MSG_TRUNC: c_int = 0x0010;
pub const MSG_CTRUNC: c_int = 0x0020;
pub const MSG_WAITALL: c_int = 0x0040;
pub const MSG_DONTWAIT: c_int = 0x0080;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const IPPROTO_TCP: c_int = 6;
/// The IPv6 membership options are Linux's; minix's socket layer does not
/// implement them, but `std`'s socket module names them.
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const TCP_NODELAY: c_int = 1;
pub const SOMAXCONN: c_int = 128;
/// `EAI_SYSTEM` (netdb): the error is in `errno`.
pub const EAI_SYSTEM: c_int = -11;

unsafe extern "C" {
    pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    pub fn socketpair(domain: c_int, ty: c_int, protocol: c_int, fds: *mut c_int) -> c_int;
    pub fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    pub fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    pub fn listen(fd: c_int, backlog: c_int) -> c_int;
    pub fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn shutdown(fd: c_int, how: c_int) -> c_int;
    pub fn setsockopt(
        fd: c_int,
        level: c_int,
        name: c_int,
        value: *const c_void,
        len: socklen_t,
    ) -> c_int;
    pub fn send(fd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn recv(fd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn sendto(
        fd: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    pub fn recvfrom(
        fd: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut sockaddr,
        addrlen: *mut socklen_t,
    ) -> ssize_t;
    pub fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn gai_strerror(code: c_int) -> *const c_char;
}
