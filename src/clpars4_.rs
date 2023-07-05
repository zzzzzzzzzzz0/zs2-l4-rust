use zhscript2::{u_::*, u2_::clpars_::*, as_ref__, as_mut_ref__};
use std::{fmt};
use regex::Regex;

struct Typr_ {
	r_:Regex
}
impl Typr_ {
	fn c__() -> char {'r'}
}
impl Typ2_ for Typr_ {
	fn s__(&self) -> &'static str {"Rege"}
	fn c__(&self) -> char {Self::c__()}
	fn can_split__(&self) -> bool {false}
	fn with__(&self, txt:&str, _:&str, argv:&mut Vec<String>) -> bool {
		if self.r_.is_match(txt) {
			for cm in self.r_.captures_iter(txt) {
				for i in 1..cm.len() {
					cm.get(i).map(|m| argv.push(m.as_str().to_string()));
				}
			}
			true
		} else {false}
	}
}
impl fmt::Debug for Typr_ {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {Typ2_::fmt(self, f)}}

struct Typh_ {}
impl Typh_ {
	fn c__() -> char {'h'}
}
impl Typ2_ for Typh_ {
	fn s__(&self) -> &'static str {"Help"}
	fn c__(&self) -> char {Self::c__()}
	fn with__(&self, tag:&str, i:&str, _:&mut Vec<String>) -> bool {tag == i}
}
impl fmt::Debug for Typh_ {fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {Typ2_::fmt(self, f)}}

pub fn set2__(argv:&Vec<String>) -> Result<List_, String> {
	let len = argv.len();
	if len < 1 || len % 4 != 1 {
		return Err(format!("参数数目不匹配 ({})", len))
	}
	let mut cp = List_::new();
	cp.rem_ = argv[0].to_string();
	let mut i = 1;
	while i < len {
		let tag = &argv[i];
		let mut typ = Typ_::Full;
		let s2 = &argv[i + 2];
		let mut argc2 = 0;
		let argc = if let Ok(i) = s2.parse::<usize>() {
			i
		} else {
			     if s2.starts_with('b') {typ = Typ_::Starts;}
			else if s2.starts_with('e') {typ = Typ_::Ends;}
			else if s2.starts_with('c') {typ = Typ_::Has;}
			else if s2.starts_with(Typr_::c__())
			      || s2.starts_with('g') {
				let mut tag = tag.to_string();
				if s2.starts_with(Typr_::c__()) {
					if !tag.starts_with('^') {
						tag.insert(0, '^')
					}
					if !tag.ends_with('$') {
						tag.push('$')
					}
				}
				match Regex::new(&tag) {
					Ok(r_) => typ = Typ_::X(Box::new(Typr_ {r_})),
					Err(e) => return Err(e.to_string())
				}
			}
			else if s2.starts_with(Typh_::c__()) {typ = Typ_::X(Box::new(Typh_ {}))}
			else if s2.starts_with('a') {
				argc2 = ARGC_Z_
			} else {
				return Err(format!("类型 {} 不支持", s2))
			}
			let s2 = &s2[1..];
			if !s2.is_empty() {
				if let Ok(i) = s2.parse::<usize>() {
					i
				} else if s2.starts_with('a') && argc2 == 0 {
					argc2 = ARGC_Z_;
					let s2 = &s2[1..];
					if !s2.is_empty() {
						if let Ok(i) = s2.parse::<usize>() {
							i
						} else {
							return Err(s2.to_string())
						}
					} else {0}
				} else {
					return Err(s2.to_string())
				}
			} else {0}
		};
		let rem = &argv[i + 1];
		let code = &argv[i + 3];
		cp.a_.push(Item_::new__(&tag, argc + argc2, typ, Cb_::S(if !code.is_empty() {code} else {rem}.to_string()), rem));
		i += 4
	}
	Ok(cp)
}

pub fn set__(argv:&Vec<String>, ret:&mut result_::List_) -> Result2_ {
	match set2__(argv) {
		Ok(cp) => {
			ret.add_obj__(Box::new(cp));
			ok__()
		}
		Err(e) => result2_::err__(e)
	}
}

pub fn par__(obj_i:usize, obj_i2:usize, help_i:usize, env:&code_::Env_) -> Result<(), (i32, Result2_)> {
	let q = as_ref__!(env.q);
	let args = &as_ref__!(q.args_);
	#[cfg(debug_assertions)]
	if as_ref__!(env.w).dbg_.arg_ {
		as_ref__!(env.w).dbg_.arg__(args);
	}

	let lou2 = RefCell_::new(true);
	let test = RefCell_::new(false);
	let pause = RefCell_::new(vec![]);
	let i_add = RefCell_::new(0);
	let mut ret2 = ok__();
	let mut ret3:Result_ = Ok(());
	if obj_i < args.len() && args.obj__(obj_i, |cp:&List_| {
		let args = args.to_vec__();
		let helpo = args.len();
		{
			let cp = List_::new2(vec![
				Item_::new("-漏|-试"),
				Item_::new2c("-暂停", 1),
			]);
			let _ = cp.for3__(&mut args.clone().into_iter().skip(obj_i2), |tag, argv, _, _, _, _| {
				match tag {
					"-漏" =>
						*as_mut_ref__!(lou2) = false,
					"-试" =>
						*as_mut_ref__!(test) = true,
					"-暂停" => as_mut_ref__!(pause).push(argv[0].clone()),
					_ => return 0
				}
				*as_mut_ref__!(i_add) += 1 + argv.len();
				0
			}, |_| 1);
		}
		let other = |tag:&str, lou| {
			match tag {
				"-h" | "-help" | "--help" | "=h" | "=help" | "#" => HELP_,
				_ => if lou && *as_ref__!(lou2) {TAG_NO_} else {0}
			}
		};
		let mut cb = |tag:&str, argv:&Vec<String>, argc0, item:&Item_, i3, no0| {
			if let Cb_::S(src) = &item.cb_ {
				if src.is_empty() {
					return other(tag, false)
				}
				if *as_ref__!(test) || as_ref__!(pause).iter().any(|i| item.tagv_.contains(i)) {
					return 0
				}
				let mut q = Qv_::new2(as_ref__!(env.q).up_.clone());
				{
					let args = &mut as_mut_ref__!(q.args_);
					for i in argv {
						if !args.is_empty() {
							as_ref__!(env.w).dunhao__(args);
						}
						args.add__(i)
					}
				}
				q.arg0_ = tag.to_string();

				let mut fa2 = code_::attr_::Item_::new(env.fa.clone());
				fa2.add__("flag", tag);
				fa2.add__("rem", &item.rem_);
				{
					let mut s = String::new();
					typ__(&item.typ_, &mut s);
					fa2.add__("type", s);
				}
				fa2.add__("code", src);
				fa2.add__("no", i3);
				fa2.add__("tag", &item.tag_);
				fa2.add__("argc", item.argc_);
				fa2.add__("i", no0);
				fa2.add__("from", argc0 + 1);

				ret2 = eval_::hello__(&src, &mut code_::Env_::new4(code_::attr_::i__(Some(fa2)), t__(q.clone()), env));
				if ret2.is_ok() {0} else {1}
			} else {
				other(tag, true)
			}
		};
		if obj_i + help_i + *as_ref__!(i_add) == helpo {
			for item in &cp.a_ {
				if let Typ_::X(x) = &item.typ_ {
					if x.c__() == Typh_::c__() {
						let i = cb(item.tag1__(), &vec![], 0, item, 1, 0);
						if i != 0 {
							ret3 = Err((i, cp.help__()));
							if i != HELP_ {
								return false
							}
						}
					}
				}
			}
			return true
		}
		ret3 = cp.for3__(&mut args.into_iter().skip(obj_i2 + *as_ref__!(i_add)), |tag, argv, argc0, item, i3, no0| {
			cb(tag, argv, argc0, item, i3, no0)
		}, |tag| {
			other(tag, true)
		});
		true
	}) {
		if let Err((ret3, s)) = ret3 {
			if *as_ref__!(test) {
				as_mut_ref__!(env.ret).add__(ret3);
				return Ok(())
			}
			return Err((ret3, world_::clpars_ret2__(ret3, s, ret2, env.w.clone())))
		}
		Ok(())
	} else {
		Err((2, if ret2.is_err() {ret2} else {result2_::err2__("第个非对象")}))
	}
}

fn typ__(t:&Typ_, s:&mut String) {
	match t {
		Typ_::Full => {}
		_ => s.push(match t {
			Typ_::Starts => 'b',
			Typ_::Ends => 'e',
			Typ_::Has => 'c',
			Typ_::X(x) => x.c__(),
			_ => panic!()
		})
	}
}

pub fn help__(from:usize, q:qv_::T_, ret:&mut result_::List_) {
	let q = as_ref__!(q);
	let args = &as_ref__!(q.args_);
	for i in from..args.len() {
		if !args.obj__(i, |cp:&List_| {
			ret.add__(cp.help__());
			true
		}) {
			let mut ret2 = String::new();
			as_ref__!(args[i]).s__(&mut ret2);
			ret.add__(ret2);
		}
	}
}
pub fn help2__(env:&code_::Env_) -> Result2_ {
	let q = env.q.clone();
	let q = as_ref__!(q);
	let args = &as_ref__!(q.args_);

	let mut src = String::new();
	as_ref__!(args[0]).s__(&mut src);

	let ret = &mut as_mut_ref__!(env.ret);
	let mut ret3 = ok__();
	for i in 1..args.len() {
		if !args.obj__(i, |cp:&List_| {
			'for1:
			for i in cp.a_.iter() {
				for i2 in &i.tagv_ {
					let q2 = Qv_::new2(q.up_.clone());
					{
						let args = &mut as_mut_ref__!(q2.args_);
						args.add__(&i2);
						as_ref__!(env.w).dunhao__(args);
						{
							let mut s = String::new();
							typ__(&i.typ_, &mut s);
							args.add__(s);
						}
						as_ref__!(env.w).dunhao__(args);
						if i.argc_ >= ARGC_Z_ {
							args.add__("a");
						} else {
							args.add__(&i.argc_);
						}
						as_ref__!(env.w).dunhao__(args);
						args.add__(&i.rem_);
					}
					let ret4 = t__(result_::List_::new());
					ret3 = eval_::hello__(&src, &mut code_::Env_::new9(t__(q2), ret4.clone(), env));
					let (is_err, only_b) = jump_::for_err__(&ret3);
					for i in &as_ref__!(ret4).a_ {
						ret.add4__(i.clone());
					}
					if !is_err {
						ret3 = ok__();
					}
					if only_b {
						break 'for1;
					}
				}
			}
			true
		}) {
			let mut ret2 = String::new();
			as_ref__!(args[i]).s__(&mut ret2);
			ret.add__(ret2);
		}
		if ret3.is_err() {
			break;
		}
	}
	ret3
}
