// 1.61.0-nightly 或其前某个极近版本方使本码达意
use core::arch::asm;

#[allow(named_asm_labels)]
pub fn hello__(p:super::Ptr_, args2: &Vec<usize>, fargs2: &Vec<f64>,
		ret_low: &mut usize, ret_high: &mut usize, ret_float: &mut f64) {
	let args = args2.as_ptr().wrapping_offset(args2.len() as isize - 1);
	let len = args2.len();
	let fargs = fargs2.as_ptr().wrapping_offset(fargs2.len() as isize - 1);
	let flen = fargs2.len();
	let lo:u64;
	let hi:u64;
	let f:f64;
	unsafe {
		asm!(
			r"
			lea    rdi, [rip + lfs]
			movsxd rsi, dword ptr [rdi + r11 * 4]
			add    rsi, rdi
			jmp    rsi
			lfs:
			.long lf0 - lfs
			.long lf1 - lfs
			.long lf2 - lfs
			.long lf3 - lfs
			.long lf4 - lfs
			.long lf5 - lfs
			.long lf6 - lfs
			.long lf7 - lfs
			.long lf8 - lfs
			lf8:
			movsd xmm7, qword ptr [r10]
			sub   r10, 8
			lf7:
			movsd xmm6, qword ptr [r10]
			sub   r10, 8
			lf6:
			movsd xmm5, qword ptr [r10]
			sub   r10, 8
			lf5:
			movsd xmm4, qword ptr [r10]
			sub   r10, 8
			lf4:
			movsd xmm3, qword ptr [r10]
			sub   r10, 8
			lf3:
			movsd xmm2, qword ptr [r10]
			sub   r10, 8
			lf2:
			movsd xmm1, qword ptr [r10]
			sub   r10, 8
			lf1:
			movsd xmm0, qword ptr [r10]
			lf0:
			lea    r12, [r15 - 6]
			cmp    r15, 6
			mov    rdi, 0
			cmovbe r12, rdi
			jbe    lp6
			test   r12, 1
			je     lp
			sub    rsp, 8
			lp:
			push   qword ptr [r14]
			sub    r14, 8
			sub    r15, 1
			cmp    r15, 6
			jne    lp
			lp6:
			lea    rdi, [rip + ls]
			movsxd rsi, dword ptr [rdi + r15 * 4]
			add    rsi, rdi
			jmp    rsi
			ls:
			.long l0 - ls
			.long l1 - ls
			.long l2 - ls
			.long l3 - ls
			.long l4 - ls
			.long l5 - ls
			.long l6 - ls
			l6:
			mov  r9, qword ptr [r14]
			sub  r14, 8
			l5:
			mov  r8, qword ptr [r14]
			sub  r14, 8
			l4:
			mov  rcx, qword ptr [r14]
			sub  r14, 8
			l3:
			mov  rdx, qword ptr [r14]
			sub  r14, 8
			l2:
			mov  rsi, qword ptr [r14]
			sub  r14, 8
			l1:
			mov  rdi, qword ptr [r14]
			l0:
			mov  rax, r11
			//push
			mov r15, r12
			call r13
			//pop
			mov r12, r15
			lea  rsp, [rsp + r12 * 8]
			test r12, 1
			je   lna
			add  rsp, 8
			lna:
			",
			in("r13") p,
			in("r14") args,
			in("r15") len,
			in("r10") fargs,
			in("r11") flen,
			lateout("rax") lo,
			lateout("rdx") hi,
			lateout("xmm0") f,
		);
	}
	*ret_low  = lo as usize;
	*ret_high = hi as usize;
	*ret_float = f;
}
