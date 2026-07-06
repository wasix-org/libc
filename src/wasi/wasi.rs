//! Definitions specific to upstream [wasi-libc](https://github.com/WebAssembly/wasi-libc),
//! shared by all non-WASIX wasi targets. The WASIX counterparts live in `wasix.rs`.

use crate::prelude::*;

pub type sigset_t = c_uchar;

s_paren! {
    // in wasi-libc clockid_t is const struct __clockid* (where __clockid is an opaque struct),
    // but that's an implementation detail that we don't want to have to deal with
    #[repr(transparent)]
    #[allow(dead_code)]
    pub struct clockid_t(*const u8);
}

unsafe impl Send for clockid_t {}
unsafe impl Sync for clockid_t {}

// FIXME(msrv): `addr_of!(EXTERN_STATIC)` is now safe; remove `unsafe` when MSRV >= 1.82
#[allow(unused_unsafe)]
pub static CLOCK_MONOTONIC: clockid_t = unsafe { clockid_t(core::ptr::addr_of!(_CLOCK_MONOTONIC)) };
#[allow(unused_unsafe)]
pub static CLOCK_REALTIME: clockid_t = unsafe { clockid_t(core::ptr::addr_of!(_CLOCK_REALTIME)) };

extern "C" {
    static _CLOCK_MONOTONIC: u8;
    static _CLOCK_REALTIME: u8;
}
