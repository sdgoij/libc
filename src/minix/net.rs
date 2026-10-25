//! Sockets (`tools/c-include/sys/socket.h`, `netinet/in.h`) and the names
//! `std` and `rustix` resolve from them.
//!
//! The net server implements the AF_UNIX and AF_INET families; `setsockopt` is
//! a no-op in the port, so most option values are declared for the consumers
//! that name them rather than for behaviour they select. The numbering follows
//! Linux's, which is what the port's headers already use (`MSG_*`,
//! `SCM_RIGHTS`, `SO_PEERCRED`, `AF_INET6 = 10`), and the `sockaddr_in` layout
//! is the plain POSIX one (family first) for the same reason: the port's only
//! decoder is `minix-libc`'s, so header and decoder move together.

use crate::prelude::*;
use crate::{c_char, c_int, c_uint, c_void, iovec, size_t, ssize_t};

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
    pub s_addr: c_uint,
}

/// `struct in6_addr`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

/// `struct sockaddr_in`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

/// `struct sockaddr_in6`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

/// `struct ip_mreq` (`netinet/in.h`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

/// `struct ipv6_mreq` (`netinet/in.h`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: c_uint,
}

/// `struct linger`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int,
}

/// `struct cmsghdr` — 12 bytes with 4-byte alignment, as the header defines it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct cmsghdr {
    pub cmsg_len: socklen_t,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

/// `struct msghdr`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: c_int,
    pub msg_control: *mut c_void,
    pub msg_controllen: socklen_t,
    pub msg_flags: c_int,
}

/// `CMSG_ALIGN` — the header's `(n + 3) & ~3`.
#[inline]
pub const fn CMSG_ALIGN(len: socklen_t) -> socklen_t {
    (len + 3) & !3
}

/// `CMSG_DATA`
///
/// # Safety
///
/// `cmsg` must point to a `cmsghdr` inside a buffer sized by `CMSG_SPACE`.
#[inline]
pub unsafe fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut u8 {
    (cmsg as *mut u8).add(CMSG_ALIGN(size_of::<cmsghdr>() as socklen_t) as usize)
}

/// `CMSG_SPACE`
#[inline]
pub const unsafe fn CMSG_SPACE(len: socklen_t) -> socklen_t {
    CMSG_ALIGN(size_of::<cmsghdr>() as socklen_t) + CMSG_ALIGN(len)
}

/// `CMSG_LEN`
#[inline]
pub const unsafe fn CMSG_LEN(len: socklen_t) -> socklen_t {
    CMSG_ALIGN(size_of::<cmsghdr>() as socklen_t) + len
}

/// `CMSG_FIRSTHDR`
///
/// # Safety
///
/// `mhdr` must point to an `msghdr` whose control buffer, if any, is valid for
/// `msg_controllen` bytes.
#[inline]
pub unsafe fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
    if unsafe { (*mhdr).msg_controllen } >= size_of::<cmsghdr>() as socklen_t {
        unsafe { (*mhdr).msg_control as *mut cmsghdr }
    } else {
        core::ptr::null_mut()
    }
}

/// `CMSG_NXTHDR`
///
/// # Safety
///
/// As `CMSG_FIRSTHDR`, and `cmsg` must be a header inside the same buffer.
#[inline]
pub unsafe fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let control = unsafe { (*mhdr).msg_control as *mut u8 };
    let controllen = unsafe { (*mhdr).msg_controllen } as usize;
    let cmsg_len = unsafe { (*cmsg).cmsg_len };
    let next = unsafe { (cmsg as *mut u8).add(CMSG_ALIGN(cmsg_len) as usize) };
    let header_len = CMSG_ALIGN(size_of::<cmsghdr>() as socklen_t) as usize;
    if (next as usize) + header_len > control as usize + controllen {
        core::ptr::null_mut()
    } else {
        next as *mut cmsghdr
    }
}

pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_AX25: c_int = 3;
pub const AF_IPX: c_int = 4;
pub const AF_APPLETALK: c_int = 5;
pub const AF_NETROM: c_int = 6;
pub const AF_BRIDGE: c_int = 7;
pub const AF_ATMPVC: c_int = 8;
pub const AF_X25: c_int = 9;
pub const AF_INET6: c_int = 10;
pub const AF_ROSE: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_NETBEUI: c_int = 13;
pub const AF_SECURITY: c_int = 14;
pub const AF_KEY: c_int = 15;
pub const AF_NETLINK: c_int = 16;
pub const AF_PACKET: c_int = 17;
pub const AF_ASH: c_int = 18;
pub const AF_ECONET: c_int = 19;
pub const AF_ATMSVC: c_int = 20;
pub const AF_RDS: c_int = 21;
pub const AF_SNA: c_int = 22;
pub const AF_IRDA: c_int = 23;
pub const AF_PPPOX: c_int = 24;
pub const AF_WANPIPE: c_int = 25;
pub const AF_LLC: c_int = 26;
pub const AF_CAN: c_int = 29;
pub const AF_TIPC: c_int = 30;
pub const AF_BLUETOOTH: c_int = 31;
pub const AF_IUCV: c_int = 32;
pub const AF_RXRPC: c_int = 33;
pub const AF_ISDN: c_int = 34;
pub const AF_PHONET: c_int = 35;
pub const AF_IEEE802154: c_int = 36;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_SEQPACKET: c_int = 5;
/// `SOCK_CLOEXEC` / `SOCK_NONBLOCK` are ORed into the type, as on Linux.
pub const SOCK_CLOEXEC: c_int = 0o2_000_000;
/// `SOCK_NONBLOCK`
pub const SOCK_NONBLOCK: c_int = 0o4_000;

/// Minix's `SOL_SOCKET` is `0xffff` (`sys/sys/socket.h`), not Linux's 1.
pub const SOL_SOCKET: c_int = 0xffff;

pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_LINGER: c_int = 13;
pub const SO_REUSEPORT: c_int = 15;
pub const SO_PEERCRED: c_int = 17;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_DOMAIN: c_int = 39;

pub const MSG_OOB: c_int = 0x0001;
pub const MSG_PEEK: c_int = 0x0002;
pub const MSG_DONTROUTE: c_int = 0x0004;
pub const MSG_EOR: c_int = 0x0008;
pub const MSG_TRUNC: c_int = 0x0010;
pub const MSG_CTRUNC: c_int = 0x0020;
pub const MSG_WAITALL: c_int = 0x0040;
pub const MSG_DONTWAIT: c_int = 0x0080;
pub const MSG_CONFIRM: c_int = 0x0800;
pub const MSG_ERRQUEUE: c_int = 0x2000;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_MORE: c_int = 0x8000;
pub const MSG_CMSG_CLOEXEC: c_int = 0x4000_0000;

/// `SCM_RIGHTS` (`sys/socket.h`)
pub const SCM_RIGHTS: c_int = 0x01;
/// `SCM_CREDS`
pub const SCM_CREDS: c_int = 0x04;

pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_IGMP: c_int = 2;
pub const IPPROTO_IPIP: c_int = 4;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_EGP: c_int = 8;
pub const IPPROTO_PUP: c_int = 12;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IDP: c_int = 22;
pub const IPPROTO_TP: c_int = 29;
pub const IPPROTO_DCCP: c_int = 33;
pub const IPPROTO_IPV6: c_int = 41;
pub const IPPROTO_ROUTING: c_int = 43;
pub const IPPROTO_FRAGMENT: c_int = 44;
pub const IPPROTO_RSVP: c_int = 46;
pub const IPPROTO_GRE: c_int = 47;
pub const IPPROTO_ESP: c_int = 50;
pub const IPPROTO_AH: c_int = 51;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_MTP: c_int = 92;
pub const IPPROTO_BEETPH: c_int = 94;
pub const IPPROTO_ENCAP: c_int = 98;
pub const IPPROTO_PIM: c_int = 103;
pub const IPPROTO_COMP: c_int = 108;
pub const IPPROTO_SCTP: c_int = 132;
pub const IPPROTO_MH: c_int = 135;
pub const IPPROTO_UDPLITE: c_int = 136;
pub const IPPROTO_MPLS: c_int = 137;
pub const IPPROTO_RAW: c_int = 255;
/// `IPPROTO_MPTCP` — beyond `u8`, as on Linux.
pub const IPPROTO_MPTCP: c_int = 262;

pub const IP_TTL: c_int = 2;
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;

pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
/// `IPV6_ADD_MEMBERSHIP` / `IPV6_DROP_MEMBERSHIP` (the `IPV6_JOIN_GROUP`
/// spelling shares the value).
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_TCLASS: c_int = 67;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const SOMAXCONN: c_int = 128;
/// `EAI_SYSTEM` (netdb): the error is in `errno`.
pub const EAI_SYSTEM: c_int = -11;

/// `TCP_KEEPIDLE`
pub const TCP_KEEPIDLE: c_int = 4;

unsafe extern "C" {
    pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    pub fn socketpair(domain: c_int, ty: c_int, protocol: c_int, fds: *mut c_int) -> c_int;
    pub fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    pub fn connect(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    pub fn listen(fd: c_int, backlog: c_int) -> c_int;
    pub fn accept(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn accept4(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t, flags: c_int) -> c_int;
    pub fn shutdown(fd: c_int, how: c_int) -> c_int;
    pub fn setsockopt(
        fd: c_int,
        level: c_int,
        name: c_int,
        value: *const c_void,
        len: socklen_t,
    ) -> c_int;
    pub fn getsockopt(
        fd: c_int,
        level: c_int,
        name: c_int,
        value: *mut c_void,
        len: *mut socklen_t,
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
    pub fn sendmsg(fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    pub fn getpeername(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    pub fn gai_strerror(code: c_int) -> *const c_char;
}
