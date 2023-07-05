#![cfg_attr(any(feature = "use_rusty_asm", feature = "use_llvm_asm"), feature(proc_macro_hygiene))]
#![cfg_attr(any(feature = "use_rusty_asm", feature = "use_llvm_asm"), feature(/*llvm_asm,*/ asm))]
//#![feature(proc_macro_hygiene)]
//#![feature(llvm_asm, asm)]
//#![feature(asm_sym)]

use zhscript2::{as_ref__, as_mut_ref__};

#[cfg(not(feature = "no_dl"))]
mod dl_;
#[cfg(not(feature = "no_dl"))]
mod func_;
#[cfg(not(feature = "no_dl"))]
mod call_;
#[cfg(not(feature = "no_dl"))]
pub mod funcs_;
pub mod g_;
pub mod i_;

pub mod clpars4_;
pub mod thread4_;
pub mod regexpr4_;
pub mod forqv_;
