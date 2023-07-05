use zhscript2::u_::*;
use super::*;

pub const NAME_: &str = "调用";

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kw: keyword_::RI_) -> Self {
		Self {super_:code_::Item1_::new(&kw), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		if a.is_empty() {
			return result2_::qve__();
		}
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a_).hello__(&code_::Env_::new6(ret2.clone(), env))?;
		let ret2 = as_ref__!(ret2);
		let mut argv2 = vec![];
		let mut start = 1;
		while start < ret2.len() {
			let i = as_ref__!(ret2[start]);
			if i.dunhao__() {
				start += 1;
				break
			}
			let mut s = String::new();
			i.s__(&mut s);
			if let Err(e) = funcs_::typ_::pargs2__(&s, &mut argv2) {
				return result2_::err__(e);
			}
			start += 1
		}
		let mut ret3 = ok__();
		if !ret2.obj__(0, |fi:&funcs_::RI_| {
			ret3 = fi.call__(&argv2, &ret2, start, env);
			ret3.is_ok()
		}) && ret3.is_ok() {
			return result2_::err__(format!("第一个非{}变量", super::func_::NAME_))
		}
		ret3
	}
}
