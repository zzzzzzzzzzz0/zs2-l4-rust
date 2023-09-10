use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use std::{ffi::{CStr, CString}};
use super::I8_;

pub const NO_ERR_RETURN:i32 = -1;
pub const NO_ERR_BREAK:i32 = -2;

pub fn cb__(env0:*mut code_::Env_, ret0:*mut Result2_, errret2:*mut i32, ctl:u8, s:*const I8_, argc:i32, argv:&mut &mut I8_) -> *mut I8_ {
	let env;
	unsafe {
		env = &mut *env0;
	}
	let q = Qv_::new2(Some(env.q.clone()));
	{
		let argv = args__(argc, argv);
		if let Ok(mut argv) = argv {
			let mut args = as_mut_ref__!(q.args_);
			for (i, s) in argv.iter_mut().enumerate() {
				if i > 0 {
					as_ref__!(env.w).dunhao__(&mut args);
				}
				args.add__(s);
			}
		}
	}
	let mut s = s__(s);
	if !s.is_empty() {
		let ret = eval_::hello__(&s, &code_::Env_::new2(t__(q), env));
		s.clear();
		if ret.is_ok() {
			as_ref__!(env.ret).s2__(ctl, &mut s);
		} else {
			ret2__(errret2, ret__(ret, ret0));
		}
	}
	CString::new(s).unwrap().into_raw()
}

pub fn free__(s:*mut I8_) {
	if !s.is_null() {
		unsafe {
			drop(CString::from_raw(s));
		}
	}
}

pub fn add__(s:*const I8_, dunhao:bool, env0:*mut code_::Env_) {
	let env;
	unsafe {
		env = &mut *env0;
	}
	let ret = &mut as_mut_ref__!(env.ret);
	if dunhao {
		as_ref__!(env.w).dunhao__(ret);
	}
	ret.add__(&s__(s));
}

pub fn args__(argc:i32, argv:&mut &mut I8_) -> Result<Vec<String>, std::str::Utf8Error> {
	unsafe {
		std::slice::from_raw_parts(argv, argc as usize)
			.iter()
			.map(|s| Ok(s__(*s)))
			.collect()
	}
}

pub fn s__(s0:*const I8_) -> String {
	if s0.is_null() {
		"NULL".to_string()
	} else {
		let s2;
		unsafe {
			s2 = CStr::from_ptr(s0);
		}
		s2.to_string_lossy().to_string()
	}
}

pub fn ret__(ret:Result2_, ret0:*mut Result2_) -> i32 {
	if let Err((i, i2, s, s2)) = ret {
		match i {
			jump_::QUIT_ => if i2 != jump_::NO_ {
				if i2 == 0 {return NO_ERR_RETURN}
				eprint!("{}", s);
				return i2
			}
			jump_::RETURN_ => if s.is_empty() {return NO_ERR_RETURN}
			jump_::BREAK_  | jump_::CONTINUE_ |
			jump_::BREAK2_ | jump_::CONTINUE2_ => {
				if !ret0.is_null() {
					let ret1;
					unsafe {
						ret1 = &mut *ret0;
					}
					let i2 = if !s.is_empty() {
						NO_ERR_BREAK
					} else {i};
					*ret1 = Err((i, i2, s, s2));
					return i2
				}
			}
			_ => {}
		}
		result2_::eprtn__(i, i2, &s, &s2);
		result2_::exitcode__(i)
	} else {0}
}
pub fn ret2__(errret2:*mut i32, err:i32) {
	if !errret2.is_null() {
		unsafe {
			*errret2 = err;
		}
	}
}
