use zhscript2::u_::*;
use super::*;

pub const NAME_: &str = "函数集";

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
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let ret2 = t__(result_::List_::new());
		t_::o__(&self.a_).hello__(&code_::Env_::new6(ret2.clone(), env))?;
	
		let v = as_ref__!(ret2).to_vec__();
		match funcs_::List_::new(&v) {
			Ok(funcs) => {
				as_mut_ref__!(env.q).add_obj_mut__(Box::new(funcs));
				ok__()
			}
			Err(e) => result2_::err__(e)
		}
	}
}
