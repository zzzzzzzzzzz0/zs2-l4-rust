use zhscript2::{u_::*, *};
use super::*;

pub const NAME_: &str = "函数";

pub struct Item_ {
	super_:set_::Item_,
}

impl Item_ {
	pub fn new(kw: keyword_::RI_, kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kw, &kws.equ_)}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {self.super_.add__(a)}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, env:&code_::Env_) -> Result2_ {
		let names = t__(result_::List_::new());
		let vals = t__(result_::List_::new());
		self.super_.hello2_1__(names.clone(), vals.clone(), env)?;
		let names = as_ref__!(names).to_vec__();
		let vals = as_ref__!(vals).to_vec__();

		let ret = if !vals.is_empty() && vals[0].starts_with("*") {
			let s = vals[0][1..].to_string();
			if let Ok(l) = s.parse::<u64>() {
				match funcs_::Item_::new(l as funcs_::Ptr_, if vals.len() > 1 {&vals[1]} else {""}) {
					Ok(i) => Ok(Rc_::new(i)),
					Err(s) => Err(s),
				}
			} else {
				Err(s)
			}
		} else {
			let funcs = match as_mut_ref__!(env.q).obj_mut__(0) {
				Some(o) => o,
				None => return result2_::err2__(dl_::NAME_)
			};
			let mut funcs = as_mut_ref__!(funcs);
			let funcs = funcs.downcast_mut::<funcs_::List_>().unwrap();
			funcs.add__(&vals, &names)
		};
		match ret {
			Ok(i) => {
				let val = result_::oi__(Box::new(i));
				if names.is_empty() {
					as_mut_ref__!(env.ret).add4__(val)
				} else {
					for i in 0..names.len() {
						qv_::val2__(&names[i], val.clone(), false, false, false, env.q.clone(), env.w.clone());
					}
				}
			}
			Err(s) => return result2_::err__(s)
		};
		ok__()
	}
}
