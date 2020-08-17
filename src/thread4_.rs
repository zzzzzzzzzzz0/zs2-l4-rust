use zhscript2::{u_::*, as_ref__};
use std::{thread};

pub fn start__(env:&code_::Env_) -> Result2_ {
	let env2 = code_::Env_::new10(env);
	thread::spawn(move || {
		if let Err((_i, s, s2)) = eval_::hello2__(&as_ref__!(env2.q).src_, |_| {}, &env2) {
			eprintln!("{} {}", s, s2)
		}
	});
	ok__()
}