//! Module that provides `aeabi` functions.
//!
//! These functions are provided by `compiler-builtins` by default, but these replacement versions are faster.  
//! Generally, the `aeabi` functions shouldn't be called directly, you should use the proper Rust equivalent
//! which will then call these functions under the hood.  
//! For example, instead of `memcpy` you should use `core::ptr::copy_nonoverlapping`.
//!
//! The `agbabi` functions may be called directly, and may be more suitable in particular situations.
//! Don't use them unless you know what you're doing.
//! Their documentation can be found [here](https://github.com/felixjones/agbabi).
#![allow(missing_docs)]

use core::arch::global_asm;

global_asm! {
    include_str!("macros.inc"),
    options(raw)
}

global_asm! {
    include_str!("memcpy.s"),
    options(raw)
}

extern "C" {
    pub fn __aeabi_memcpy8(dst: *mut u8, src: *const u8, count: usize);
    pub fn __aeabi_memcpy4(dst: *mut u8, src: *const u8, count: usize);
    pub fn __aeabi_memcpy(dst: *mut u8, src: *const u8, count: usize);
    pub fn __agbabi_memcpy2(dst: *mut u8, src: *const u8, count: usize);
    pub fn __agbabi_memcpy1(dst: *mut u8, src: *const u8, count: usize);
    pub fn memcpy(dst: *mut u8, src: *const u8, count: usize) -> *mut u8;
}

global_asm! {
    include_str!("rmemcpy.s"),
    options(raw)
}

extern "C" {
    pub fn __agbabi_rmemcpy(dst: *mut u8, src: *const u8, count: usize);
    pub fn __agbabi_rmemcpy1(dst: *mut u8, src: *const u8, count: usize);
}

global_asm! {
    include_str!("memmove.s"),
    options(raw)
}

extern "C" {
    pub fn __aeabi_memmove8(dst: *mut u8, src: *const u8, count: usize);
    pub fn __aeabi_memmove4(dst: *mut u8, src: *const u8, count: usize);
    pub fn __aeabi_memmove(dst: *mut u8, src: *const u8, count: usize);
    pub fn __agbabi_memmove1(dst: *mut u8, src: *const u8, count: usize);
    pub fn memmove(dst: *mut u8, src: *const u8, count: usize) -> *mut u8;
}

global_asm! {
    include_str!("memset.s"),
    options(raw)
}

extern "C" {
    pub fn __aeabi_memset8(dst: *mut u8, count: usize, value: i32);
    pub fn __aeabi_memset4(dst: *mut u8, count: usize, value: i32);
    pub fn __aeabi_memset(dst: *mut u8, count: usize, value: i32);
    pub fn __aeabi_memclr8(dst: *mut u8, count: usize);
    pub fn __aeabi_memclr4(dst: *mut u8, count: usize);
    pub fn __aeabi_memclr(dst: *mut u8, count: usize);
    pub fn __agbabi_wordset4(dst: *mut u8, count: usize, value: i32);
    pub fn __agbabi_lwordset4(dst: *mut u8, count: usize, value: i64);
    pub fn memset(dst: *mut u8, value: i32, count: usize) -> *mut u8;
}

// why are these needed??? rust shouldn't be calling C++ exception unwinding code
// can be fixed by enabling lto, maybe? https://blog.bokuweb.me/entry/2020/04/14/101202
#[doc(hidden)]
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {}

#[doc(hidden)]
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr1() {}
