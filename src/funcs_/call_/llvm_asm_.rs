pub fn hello__(p:super::Ptr_, args2: &Vec<usize>, fargs2: &Vec<f64>,
		ret_low: &mut usize, ret_high: &mut usize, ret_float: &mut f64) {
	let args = args2.as_ptr().wrapping_offset(args2.len() as isize - 1);
	let len = args2.len();
	let fargs = fargs2.as_ptr().wrapping_offset(fargs2.len() as isize - 1);
	let flen = fargs2.len();
	let lo:u64;
	let hi:u64;
	unsafe {
		llvm_asm!("
			lea    rdi, [rip + lfs]
			movsxd rsi, dword ptr [rdi + $7 * 4]
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
			movsd xmm7, qword ptr [$6]
			sub   $6, 8
			lf7:
			movsd xmm6, qword ptr [$6]
			sub   $6, 8
			lf6:
			movsd xmm5, qword ptr [$6]
			sub   $6, 8
			lf5:
			movsd xmm4, qword ptr [$6]
			sub   $6, 8
			lf4:
			movsd xmm3, qword ptr [$6]
			sub   $6, 8
			lf3:
			movsd xmm2, qword ptr [$6]
			sub   $6, 8
			lf2:
			movsd xmm1, qword ptr [$6]
			sub   $6, 8
			lf1:
			movsd xmm0, qword ptr [$6]
			lf0:
			lea    r12, [$5 - 6]
			cmp    $5, 6
			mov    rdi, 0
			cmovbe r12, rdi
			jbe    lp6
			test   r12, 1
			je     lp
			sub    rsp, 8
			lp:
			push   qword ptr [$4]
			sub    $4, 8
			sub    $5, 1
			cmp    $5, 6
			jne    lp
			lp6:
			lea    rdi, [rip + ls]
			movsxd rsi, dword ptr [rdi + $5 * 4]
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
			mov r9, qword ptr [$4]
			sub $4, 8
			l5:
			mov r8, qword ptr [$4]
			sub $4, 8
			l4:
			mov rcx, qword ptr [$4]
			sub $4, 8
			l3:
			mov rdx, qword ptr [$4]
			sub $4, 8
			l2:
			mov rsi, qword ptr [$4]
			sub $4, 8
			l1:
			mov rdi, qword ptr [$4]
			l0:
			mov  rax, $7
			call $3
			lea  rsp, [rsp + r12 * 8]
			test r12, 1
			je   lna
			add  rsp, 8
			lna:
		"
		: "={ax}"(lo), "={dx}"(hi), "={xmm0}"(*ret_float)
		: "{r13}"(p), "{r14}"(args), "{r15}"(len), "{r10}"(fargs), "{r11}"(flen)
		: "memory", "rsp"
		: "volatile", "alignstack", "intel"
		);
	}
	*ret_low  = lo as usize;
	*ret_high = hi as usize;
}
