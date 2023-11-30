use zhscript2::{u_::*, as_ref__, as_mut_ref__};

pub fn z__(env:&code_::Env_, from:usize) -> Result2_ {
	//保有这一区用于标志等临时变量
	let mut q2 = Some(env.q.clone());
	let mut src = String::new();
	let mut next = false;
	let mut no_self = false;
	let mut inc_mod = false;
	{
		let q = as_ref__!(env.q);
		let args = &as_ref__!(q.args_);
		let a = &args.a_;
		let mut idx = from;
		let mut i2 = 0;
		while idx < a.len() {
			let i = as_ref__!(a[idx]);
			if i.dunhao__() {
				i2 += 1;
			} else {
				match i2 {
					0 => {
						match code_::qv4rem__(&i.rem_, |_| {false}, q2.unwrap(), env.w.clone()) {
							Ok(q3) => q2 = q3,
							Err(e) => return e,
						}
					}
					_ => {
						{
							let rem = &i.rem_;
							if rem.len() > 0 {
								return as_ref__!(env.w).no_rem2__(&rem[0])
							}
						}
						match i2 {
							1 => i.s__(&mut src),
							2 => {
								let mut s = String::new();
								i.s__(&mut s);
								for c in s.bytes() {
									match c {
										b'n' => next = true,
										b'o' => no_self = true,
										b'm' => inc_mod = true,
										_ => return result2_::err__(s + " 内含不支持，仅 n、o、m")
									}
								}
							}
							_ => return result2_::err__(i2.to_string() + "参超量")
						}
					}
				}
			}
			idx += 1;
		}
	}
	let mut ret = ok__();
	qv_::for3__(q2.unwrap(), env.w.clone(), |q, w, _| -> Option<()> {
		let args = t__(result_::List_::new());
		let mut f__ = || {
			let mut q4 = Qv_::new2(Some(env.q.clone()));
			q4.args_ = args.clone();
			ret = eval_::hello2__(&src, |_| {}, &code_::Env_::new2(t__(q4), env));
			if ret.is_ok() {Some(())} else {None}
		};
		let q = as_ref__!(q);
		{
			let args = &mut as_mut_ref__!(args);
			args.clear();
			args.add__("n");
			for i in &q.name_ {
				as_ref__!(w).dunhao__(args);
				args.add__(i);
			}
		}
		f__()?;
		if q.src_is_file_ {
			{
				let args = &mut as_mut_ref__!(args);
				args.clear();
				args.add__("s");
				as_ref__!(w).dunhao__(args);
				args.add__(q.src_.to_string());
			}
			f__()?;
		}
		for i in &q.vars_.a_ {
			{
				let args = &mut as_mut_ref__!(args);
				args.clear();
				args.add__("v");
				as_ref__!(w).dunhao__(args);
				let i = as_ref__!(i);
				args.add__(i.name__());
				//println!("var {:?}", i);
			}
			f__()?;
		}
		for i in &q.defs_.a_ {
			{
				let args = &mut as_mut_ref__!(args);
				args.clear();
				args.add__("d");
				as_ref__!(w).dunhao__(args);
				args.add__(as_ref__!(i).name__());
			}
			f__()?;
		}
		None
	}, next, no_self, inc_mod);
	ret
}

