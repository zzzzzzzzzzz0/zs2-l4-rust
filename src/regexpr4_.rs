use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use super::g_;
use regex::{Regex, Captures};

fn r__(s:&str) -> Result<Regex, result2_::Err_> {
	match Regex::new(s) {
		Ok(r) => Ok(r),
		Err(e) => Err((result2_::ERR_, 0, e.to_string(), "".to_string()))
	}
}

fn cb__(src:&str, cm: &Captures, get:bool, env:&code_::Env_) -> Result2_ {
	//let q = Qv_::new2(Some(env.q.clone()));
	let q = Qv_::new2(as_ref__!(env.q).up_.clone());
	{
		let args = &mut as_mut_ref__!(q.args_);
		for idx in 1..cm.len() {
			if !args.is_empty() {
				as_ref__!(env.w).dunhao__(args);
			}
			args.add__(&cm[idx]);
		}
	}
	eval_::hello__(src, &mut if get {
		code_::Env_::new2(t__(q), env)
	} else {
		code_::Env_::new9(t__(q), t__(result_::List_::new()), env)
	})
}

pub fn for__(args:&Vec<String>, get:u8, env:&code_::Env_) -> Result2_ {
	if args.len() < 3 {
		return g_::err__('<')
	}
	let r = r__(&args[1])?;
	let txt = &args[0];
	let src = &args[2];
	match get {
		10 | 11 => return repla_1__(r, txt, src, get, env),
		_ => {}
	}
	let mut i = 0;
	let mut only_b = false;
	for cm in r.captures_iter(txt) {
		i += 1;
		let ret2 = cb__(src, &cm, match get {
			0 => false,
			_ => true,
		}, env);
		/*if ret2.is_err() {
			if let Err((i, _, ref s, _)) = ret2 {
				if i == jump_::BREAK_ && s.is_empty() {break}
				if i == jump_::CONTINUE_ && s.is_empty() {continue}
			}
			return ret2
		}*/
		jump_::for_err2__(ret2, &mut only_b)?;
		if only_b {break;}
	}
	match get {
		0 => as_mut_ref__!(env.ret).add__(i.to_string()),
		_ => {}
	}
	ok__()
}

fn repla_1__(r:Regex, txt:&str, src:&str, get:u8, env:&code_::Env_) -> Result2_ {
	let mut ret2 = ok__();
	let ret = match get {
		b'1' | 11 =>
			r.replace_all(txt, |cm: &Captures| {
				let ret4 = t__(result_::List_::new());
				ret2 = cb__(src, cm, true, &code_::Env_::new6(ret4.clone(), env));
				let mut ret3 = String::new();
				as_ref__!(ret4).s2__(b'0', &mut ret3);
				ret3
			}),
		_ => r.replace_all(txt, src)
	};
	as_mut_ref__!(env.ret).add__(&ret);
	ret2
}

pub fn repla__(args:&Vec<String>, env:&code_::Env_) -> Result2_ {
	if args.len() < 4 {
		return g_::err__('<')
	}
	let r = r__(&args[2])?;
	repla_1__(r, &args[1], &args[3], match args[0].as_str() {
		"1" => b'1',
		_ => 0
	}, env)
}

pub fn test__(argv:&Vec<String>) -> Result<bool, Result2_> {
	if argv.len() < 2 {
		return Err(g_::err__('<'))
	}
	let end = argv.len() - 1;
	match Regex::new(&argv[end]) {
		Ok(re) =>
			for idx in 0..end {
				let i = &argv[idx];
				if re.is_match(i) {
					return Ok(true)
				}
			},
		Err(e) => {return Err(result2_::err__(e.to_string()))}
	}
	Ok(false)
}
