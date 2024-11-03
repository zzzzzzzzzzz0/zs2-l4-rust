use zhscript2::{u_::*, *};
use super::*;

pub const NAME_: &str = "待函数";

pub struct Item_ {
	super_:func_::Item_,
}

impl Item_ {
	pub fn new(kw: keyword_::RI_, kws:&keyword_::List_) -> Self {
		Self {super_:func_::Item_::new(kw, kws)}
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
		#[cfg(debug_assertions)]
		return self.super_.hello__(env);
		#[cfg(not(debug_assertions))]
		ok__()
	}
}
