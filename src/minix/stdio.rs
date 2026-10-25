//! `stdio.h`. Only the opaque `FILE` is needed here: `xkbcommon` passes a
//! `*mut FILE` to `xkb_compose_table_new_from_file`, and nothing names its
//! fields.

/// `__FILE`, the struct the header's `typedef` names.
#[repr(C)]
pub struct __FILE {
    _private: [u8; 0],
}

/// `FILE`, the typedef (`typedef struct __FILE FILE`).
pub type FILE = __FILE;
