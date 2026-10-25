//! Time types and calls from `tools/c-include/time.h`.

use crate::prelude::*;
use crate::{c_int, c_long, clockid_t, time_t};

/// `struct timespec`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

/// `struct timeval` (`sys/time.h`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: c_long,
}

/// `struct tm`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
}

/// `CLOCK_*`. The `*_CPUTIME_ID` clocks are Linux's; minix serves `REALTIME`
/// and `MONOTONIC`.
pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;

/// `TIMER_ABSTIME`
pub const TIMER_ABSTIME: c_int = 1;

unsafe extern "C" {
    pub fn clock_gettime(clockid: clockid_t, tp: *mut timespec) -> c_int;
    pub fn clock_settime(clockid: clockid_t, tp: *const timespec) -> c_int;
    pub fn clock_getres(clockid: clockid_t, res: *mut timespec) -> c_int;
    pub fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
}

// ---- timerfd(3) (`sys/timerfd.h`) ----

/// `struct itimerspec` — the period and the next expiry of a timer.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

pub const TFD_CLOEXEC: c_int = crate::O_CLOEXEC;
pub const TFD_NONBLOCK: c_int = crate::O_NONBLOCK;
/// `it_value` is an absolute time on the timer's clock.
pub const TFD_TIMER_ABSTIME: c_int = 1;

unsafe extern "C" {
    pub fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    pub fn timerfd_settime(
        fd: c_int,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
    pub fn timerfd_gettime(fd: c_int, curr_value: *mut itimerspec) -> c_int;
}
