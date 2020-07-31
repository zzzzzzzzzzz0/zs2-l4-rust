//https://github.com/Aloxaf/funcall

use std::mem;
use rusty_asm::rusty_asm;

pub type Ptr_ = *const fn();

pub fn call__(p:Ptr_, args2: &Vec<usize>, fargs2: &Vec<f64>,
		ret_low: &mut usize, ret_high: &mut usize, ret_float: &mut f64) {
	unsafe {
		rusty_asm! {
		    let mut low  : usize: out("{rax}");
		    let mut high : usize: out("{rdx}");
		    let mut float: f64  : out("{xmm0}"); // https://github.com/rust-lang/rust/issues/20213
		
		    let args : in("r") = args2.as_ptr().wrapping_offset(args2.len() as isize - 1);
		    let len  : in("r") = args2.len();
		    let fargs: in("r") = fargs2.as_ptr().wrapping_offset(fargs2.len() as isize - 1);
		    let flen : in("r") = fargs2.len();
		    let func : in("m") = p;
		
		    clobber("memory");
		    clobber("rsp");
		
		    clobber("rdi");
		    clobber("rsi");
		    clobber("rdx");
		    clobber("rcx");
		    clobber("r8");
		    clobber("r9");
		
		    clobber("r10");
		    clobber("r11");
		    clobber("r12");
		
		    asm("alignstack", "intel") {r"
		        lea    rdi, [rip + .LFLABELS${:uid}]
		        movsxd rsi, dword ptr [rdi + $flen * 4]
		        add    rsi, rdi
		        jmp    rsi
		
		    .LFLABELS${:uid}:
		        .long .LARG0${:uid}-.LFLABELS${:uid}
		        .long .LARG1${:uid}-.LFLABELS${:uid}
		        .long .LARG2${:uid}-.LFLABELS${:uid}
		        .long .LARG3${:uid}-.LFLABELS${:uid}
		        .long .LARG4${:uid}-.LFLABELS${:uid}
		        .long .LARG5${:uid}-.LFLABELS${:uid}
		        .long .LARG6${:uid}-.LFLABELS${:uid}
		        .long .LARG7${:uid}-.LFLABELS${:uid}
		        .long .LARG8${:uid}-.LFLABELS${:uid}
		
		    .LARG8${:uid}:
		        movsd xmm7, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG7${:uid}:
		        movsd xmm6, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG6${:uid}:
		        movsd xmm5, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG5${:uid}:
		        movsd xmm4, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG4${:uid}:
		        movsd xmm3, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG3${:uid}:
		        movsd xmm2, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG2${:uid}:
		        movsd xmm1, qword ptr [$fargs]
		        sub   $fargs, 8
		    .LARG1${:uid}:
		        movsd xmm0, qword ptr [$fargs]
		    .LARG0${:uid}:
		
		        lea    r12, [$len - 6]
		        cmp    $len, 6
		        mov    rdi, 0
		        cmovbe r12, rdi
		        jbe    .LPUSH_F6${:uid}
		
		        test   r12, 1
		        je     .LPUSH${:uid}
		        sub    rsp, 8
		    .LPUSH${:uid}:
		        push   qword ptr [$args]
		        sub    $args, 8
		        sub    $len, 1
		        cmp    $len, 6
		        jne    .LPUSH${:uid}
		
		    .LPUSH_F6${:uid}:
		        lea    rdi, [rip + .LABELS${:uid}]
		        movsxd rsi, dword ptr [rdi + $len * 4]
		        add    rsi, rdi
		        jmp    rsi
		
		    .LABELS${:uid}:
		        .long .LCALL${:uid}-.LABELS${:uid}
		        .long .L1${:uid}-.LABELS${:uid}
		        .long .L2${:uid}-.LABELS${:uid}
		        .long .L3${:uid}-.LABELS${:uid}
		        .long .L4${:uid}-.LABELS${:uid}
		        .long .L5${:uid}-.LABELS${:uid}
		        .long .L6${:uid}-.LABELS${:uid}
		
		    .L6${:uid}:
		        mov  r9, qword ptr [$args]
		        sub  $args, 8
		    .L5${:uid}:
		        mov  r8, qword ptr [$args]
		        sub  $args, 8
		    .L4${:uid}:
		        mov  rcx, qword ptr [$args]
		        sub  $args, 8
		    .L3${:uid}:
		        mov  rdx, qword ptr [$args]
		        sub  $args, 8
		    .L2${:uid}:
		        mov  rsi, qword ptr [$args]
		        sub  $args, 8
		    .L1${:uid}:
		        mov  rdi, qword ptr [$args]
		
		    .LCALL${:uid}:
		        mov  rax, $flen
		        call $func
		
		        lea  rsp, [rsp + r12 * 8]
		        test r12, 1
		        je   .LNOALIGN2${:uid}
		        add  rsp, 8
		    .LNOALIGN2${:uid}:
		    "}
		
		    *ret_low   = low;
		    *ret_high  = high;
		    *ret_float = float;
		}
	}
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
