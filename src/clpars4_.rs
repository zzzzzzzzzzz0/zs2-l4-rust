use zhscript2::{u_::*, u2_::clpars_::*, as_ref__, as_mut_ref__};
use std::{fmt};
use regex::Regex;

struct Typr_ {
	r_:Regex
}

impl Typ2_ for Typr_ {
	fn with__(&self, txt:&str, argv:&mut Vec<String>, _:&Item_) -> bool {
		if self.r_.is_match(txt) {
			for cm in self.r_.captures_iter(txt) {
				for idx in 1..cm.len() {
					argv.push(cm[idx].to_string());
				}
			}
			true
		} else {false}
	}
}
impl fmt::Debug for Typr_ {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {f.pad("Rege")}
}

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
			else if s2.starts_with('r') {
				match Regex::new(&tag) {
					Ok(r_) => typ = Typ_::X(Box::new(Typr_ {r_})),
					Err(e) => return Err(e.to_string())
				}
			}
			else if s2.starts_with('a') {
				argc2 = ARGC_Z_
			} else {
				return Err(s2.to_string())
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
		cp.a_.push(Item_::new__(tag, argc + argc2, typ, Cb_::S(if !code.is_empty() {code} else {rem}.to_string()), rem));
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

pub fn par__(obj_i:usize, obj_i2:usize, env:&code_::Env_) -> Result<(), (i32, Result2_)> {
	let q = as_ref__!(env.q);
	let args = &as_ref__!(q.args_);
	if as_ref__!(env.w).dbg_.arg_ {
		as_ref__!(env.w).dbg_.arg__(args);
	}
	let mut ret2 = ok__();
	let mut ret3 = 0;
	let ret4 = RefCell_::new(String::new());
	let lou2 = RefCell_::new(true);
	if obj_i < args.len() && args.obj__(obj_i, |cp:&List_| {
		let other__ = |tag:&str, lou| {
			match tag {
				"-h" | "--help" => 251,
				"-漏" => {
					*as_mut_ref__!(lou2) = false;
					0
				}
				_ => {
					if lou && *as_ref__!(lou2) {
						as_mut_ref__!(ret4).push_str(&as_ref__!(env.w).text__(tag));
						as_mut_ref__!(ret4).push_str("漏");
						2
					} else {0}
				}
			}
		};
		ret3 = cp.for__(&mut args.to_vec__().into_iter().skip(obj_i2), |tag, argv, item, i3| {
			if let Cb_::S(src) = &item.cb_ {
				if src.is_empty() {
					return other__(tag, false)
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
				q.src_ = tag.to_string();

				/*let fa2 = |name:&str| {
					match name {
						"1" | "flag" => ret.add__(tag),
						"tag" => ret.add__(&item.tag_),
						"2" | "rem" => ret.add__(&item.rem_),
						"3" | "type" => ret.add__(match item.typ_ {
							Typ_::Full => "",
							Typ_::Starts => "b",
							Typ_::Ends => "e",
							Typ_::Has => "c",
						}),
						"argc" => ret.add__(item.argc_),
						"4" | "code" => ret.add__(src),
						"5" | "i" => ret.add__(i3),
						_ => {}
					}
				};*/
				let mut fa2 = code_::attr_::Item_::new(env.fa.clone());
				fa2.add__("flag", tag);
				fa2.add__("rem", &item.rem_);
				fa2.add__("type", match item.typ_ {
					Typ_::Full => "",
					Typ_::Starts => "b",
					Typ_::Ends => "e",
					Typ_::Has => "c",
					Typ_::X(_) => "r",
				});
				fa2.add__("code", src);
				fa2.add__("no", i3);
				fa2.add__("tag", &item.tag_);
				fa2.add__("argc", item.argc_);

				ret2 = eval_::hello__(src, &mut code_::Env_::new4(code_::attr_::i__(Some(fa2)), t__(q.clone()), env));
				return if ret2.is_ok() {0} else {1}
			}
			other__(tag, true)
		}, |tag| {
			other__(tag, true)
		});
		true
	}) {
		if ret3 == 0 && ret2.is_ok() {Ok(())} else {
			Err((ret3, match ret3 {
				2 => result2_::err2__(&as_ref__!(ret4)),
				250 => result2_::err2__("参数不足"),
				_ => ret2
			}))
		}
	} else {Err((2, result2_::err2__("第个非对象")))}
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
