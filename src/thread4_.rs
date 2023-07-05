use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use std::{thread};

pub fn start__(argv:&Vec<String>, env:&code_::Env_) -> Result2_ {
	let mut src = String::new();
	let env2 = {
		let q2 = Qv_::new2(Some(env.q.clone()));
		{
			let mut add__ = |argv:&Vec<String>| {
				if argv.is_empty() {
					return result2_::err2__("无源")
				}
				src = argv[0].to_string();
				let mut args = as_mut_ref__!(q2.args_);
				for idx in 1..argv.len() {
					if !args.is_empty() {
						as_ref__!(env.w).dunhao__(&mut args);
					}
					args.add__(&argv[idx]);
				}
				ok__()
			};
			if argv.is_empty() {
				let q = as_ref__!(env.q);
				add__(&as_ref__!(q.args_).to_vec__())?;
			} else {
				add__(argv)?;
			}
		}
		code_::Env_::new10(&code_::Env_::new2(t__(q2), env))
	};
	let t = thread::spawn(move || {
		if let Err((i, i2, s, s2)) = eval_::hello2__(&src, |_| {}, &env2) {
			result2_::eprtn__(i, i2, &s, &s2)
		}
	});
	as_mut_ref__!(env.ret).add4__(result_::oi__(Box::new(Rc_::new(t))));
	ok__()
}

pub fn stop__(env:&code_::Env_) -> Result2_ {
	let q = as_ref__!(env.q);
	let args = &as_ref__!(q.args_);
	for i in 0..args.len() {
		if !args.obj__(i, |t:& thread::JoinHandle<()>| {
			//t.join().unwrap();
			true
		}) {
			return result2_::err2__("非线程变量")
		}
	}
	ok__()
}

