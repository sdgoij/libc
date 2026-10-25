//! MINIX bindings.
//!
//! MINIX is a Unix with its own C library (`crates/minix-libc` in the minixrs
//! tree), so it does not inherit another libc's headers. These declarations
//! mirror the port's own headers (`tools/c-include/`) and are what the crates
//! this fork exists for — `rustix`, and through it `calloop` and `smithay` —
//! name at compile time.
//!
//! The crate root selects this module *before* its `new`/`unix` trees, whose
//! family dispatch has no minix arm and whose layouts are other libcs'.

#![allow(missing_debug_implementations, missing_copy_implementations)]

mod dlfcn;
mod errno;
mod event;
mod fs;
mod io;
mod mman;
mod net;
mod process;
mod time;
mod types;

pub use dlfcn::*;
pub use errno::*;
pub use event::*;
pub use fs::*;
pub use io::*;
pub use mman::*;
pub use net::*;
pub use process::*;
pub use time::*;
pub use types::*;
