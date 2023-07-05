//https://github.com/Aloxaf/funcall

use std::mem;

pub type Ptr_ = *const fn();

#[cfg(feature = "use_rusty_asm")]
mod rusty_asm_;
#[cfg(feature = "use_rusty_asm")]
use rusty_asm_::*;

#[cfg(feature = "use_llvm_asm")]
mod llvm_asm_;
#[cfg(feature = "use_llvm_asm")]
use llvm_asm_::*;

#[cfg(all(not(feature = "use_rusty_asm"), not(feature = "use_llvm_asm")))]
mod asm_;
#[cfg(all(not(feature = "use_rusty_asm"), not(feature = "use_llvm_asm")))]
use asm_::*;

pub fn call__(p:Ptr_, args2: &Vec<usize>, fargs2: &Vec<f64>,
		ret_low: &mut usize, ret_high: &mut usize, ret_float: &mut f64) {
	hello__(p, args2, fargs2, ret_low, ret_high, ret_float);
}

pub trait IntoArg {
    fn into_arg(self) -> Vec<usize>;
}

impl<T> IntoArg for *const T {
    fn into_arg(self) -> Vec<usize> {
        vec![self as usize]
    }
}

impl<T> IntoArg for *mut T {
    fn into_arg(self) -> Vec<usize> {
        vec![self as usize]
    }
}

impl IntoArg for f32 {
    fn into_arg(self) -> Vec<usize> {
        (self as f64).into_arg()
    }
}

macro_rules! impl_intoarg {
    ($($ty:ty), *) => {
        $(impl IntoArg for $ty {
            fn into_arg(self) -> Vec<usize> {
                let len = mem::size_of::<$ty>() / mem::size_of::<usize>();
                if len <= 1 {
                    vec![self as usize]
                } else {
                    unsafe {
                        std::slice::from_raw_parts(&self as *const _ as *const usize, len).to_vec()
                    }
                }
            }
        })*
    };
}

impl_intoarg!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f64);
