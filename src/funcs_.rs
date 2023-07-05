use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use super::g_;
use std::{mem, ffi::{CStr}, str, fmt};
use libloading::{Library, os::unix as limp_};

#[cfg(feature = "i8_u8")]
type I8_ = u8;
#[cfg(not(feature = "i8_u8"))]
type I8_ = i8;

mod call_;
pub use call_::{IntoArg, Ptr_};
pub mod callback_;
pub mod typ_;
use typ_::*;

fn s__<'a>(i:*const I8_) -> &'a str {
	if i as usize == 0 {
		"NULL"
	} else {unsafe {
		CStr::from_ptr(i).to_str().unwrap()
	}}
}

pub type RI_ = Rc_<Item_>;

pub type Ptr2_ = fn(&code_::Env_) -> Result2_;
pub type Ptr3_ = fn(&[String], &mut Vec<String>) -> Result<(), String>;

#[derive(Debug)]
pub enum Ptr__ {
	Ptr,
	Ptr2,
	Ptr3,
}

pub struct Item_ {
	p_:Ptr__,
	p_0_:Option<Ptr_>,
	p_1_:Option<Ptr2_>,
	p_2_:Option<Ptr3_>,
	argv_:Vec<Typ_>,
	ret_:Typ_,
}

impl Item_ {
	pub fn new(p:Ptr_, args:&str) -> Result<Self, String> {
		Self::new__(Ptr__::Ptr, Some(p), None, None, args)
	}
	pub fn new2(p:Ptr2_, args:&str) -> Result<Self, String> {
		Self::new__(Ptr__::Ptr2, None, Some(p), None, args)
	}
	pub fn new3(p:Ptr3_, args:&str) -> Result<Self, String> {
		Self::new__(Ptr__::Ptr3, None, None, Some(p), args)
	}
	fn new__(p_:Ptr__, p_0_:Option<Ptr_>, p_1_:Option<Ptr2_>, p_2_:Option<Ptr3_>,
			args:&str) -> Result<Self, String> {
		let mut argv_ = vec![];
		let mut ret_ = Typ_::Void;
		pargs__(args, &mut ret_, &mut argv_)?;
		Ok(Self {p_, p_0_, p_1_, p_2_, argv_, ret_})
	}

	pub fn call__(&self, argv2:&Vec<Typ_>, args:&result_::List_, start:usize, env:&code_::Env_) -> Result2_ {
		match self.p_ {
			Ptr__::Ptr => {
				let args = args.to_vec5__(start);
				let mut args2: Vec<usize> = vec![];
				let mut fargs2: Vec<f64> = vec![];
				let mut args_0 = vec![];
				let mut buf = vec![0 as I8_; 512]; //1953
				let mut errret = 0;
				let errret2:*mut i32 = &mut errret;
				let mut ret2:Vec<*mut result_::List_> = vec![];
				let env2:*mut code_::Env_ = Box::into_raw(Box::new(code_::Env_::new10(env)));
				let ret3:*mut Result2_ = Box::into_raw(Box::new(ok__()));
				{
					let mut push_s__ = |i:&str, args2:&mut Vec<usize>| {
						self.push__(if i.ends_with("\0") {
							i.as_ptr()
						} else {
							let s = i.to_string() + "\0";
							args_0.push(s);
							args_0[args_0.len() - 1].as_ptr()
						}, args2)
					};
					let mut idx = 0;
					let mut args2__ = |argv:&Vec<Typ_>| {
						//println!("{:?}", argv);
						for t in argv {
							match t {
								Typ_::IntVal(i) => {
									self.push__(*i, &mut args2);
									continue
								}
								Typ_::UIntVal(i) => {
									self.push__(*i, &mut args2);
									continue
								}
								Typ_::StringVal(i) => {
									push_s__(i, &mut args2);
									continue
								}
								Typ_::Buf | Typ_::BufSiz => {
									self.push__(buf.as_mut_ptr(), &mut args2);
									if *t == Typ_::BufSiz {
										self.push__(buf.len(), &mut args2);
									}
									continue
								}
								Typ_::Errret => {
									self.push__(errret2, &mut args2);
									continue
								}
								Typ_::Z => {
									let len = args.len();
									if len > idx {
										self.push__(len - idx, &mut args2);
										while idx < len {
											push_s__(&args[idx], &mut args2);
											idx += 1;
										}
									} else {
										self.push__(0, &mut args2);
									}
									continue
								}
								Typ_::Ret => {
									let i = || {Box::into_raw(Box::new(result_::List_::new()))};
									ret2.push(i());
									self.push__(ret2[ret2.len() - 1], &mut args2);
									continue
								}
								Typ_::Callback(i) => {
									match i.as_str() {
										"" => {
											self.push__(callback_::cb__ as u64, &mut args2);
											self.push__(callback_::free__ as u64, &mut args2);
											self.push__(env2, &mut args2);
											self.push__(ret3, &mut args2);
										}
										"add" | "A" => {
											self.push__(callback_::add__ as u64, &mut args2);
											self.push__(env2, &mut args2);
										}
										_ => return no_typ__(t, ' ')
									}
									continue
								}
								_ => {}
							}
							if idx >= args.len() {
								return g_::err__('<')
							}
							let i = &args[idx];
							match t {
								Typ_::Int => {self.push2__::<i32>(i, &mut args2)?}
								Typ_::UInt => {self.push2__::<u32>(i, &mut args2)?}
								Typ_::Long => {self.push2__::<i64>(i, &mut args2)?}
								Typ_::ULong | Typ_::Addr => {self.push2__::<u64>(i, &mut args2)?}
								Typ_::Float => {self.push3__::<f64>(i, &mut fargs2, &mut args2)?}
								Typ_::Char => self.push__(if i.is_empty() {0} else {let a = i.as_bytes(); a[a.len() - 1]}, &mut args2),
								Typ_::CharPtr => push_s__(i, &mut args2),
								_ => return no_typ__(t, ' ')
							}
							idx += 1;
						}
						ok__()
					};
					args2__(&self.argv_)?;
					args2__(&argv2)?;
					if idx < args.len() {
						return g_::err__('>')
					}
				}

				let mut ret_low: usize = 0;
				let mut ret_high: usize = 0;
				let mut ret_float: f64 = 0.0;
				let p = self.p_0_.unwrap();
				call_::call__(p, &args2, &fargs2, &mut ret_low, &mut ret_high, &mut ret_float);
				let mut ret = as_mut_ref__!(env.ret);
				match self.ret_ {
					Typ_::Void => {}
					Typ_::Int => ret.add__(ret_low as i32),
					Typ_::UInt | Typ_::Unsigned => ret.add__(ret_low as u32),
					Typ_::Long => ret.add__(ret_low as i64),
					Typ_::ULong | Typ_::Addr => ret.add__(ret_low as u64),
					Typ_::Float => ret.add__(ret_float),
					Typ_::CharPtr => ret.add__(s__(ret_low as *const I8_)),
					Typ_::NBool => ret.add__(if ret_low == 0 {"1"} else {"0"}),
					_ => return no_typ__(&self.ret_, 'r')
				}
				{
					let mut by_err_use = false;
					for t in &self.argv_ {
						match t {
							Typ_::Errret => {
								if errret > 0 && errret <= result2_::ERR_ {
									return result2_::err__(format!("{} 错误码{}", s__(buf.as_ptr()), errret))
								}
								by_err_use = true;
							}
							Typ_::Buf | Typ_::BufSiz => {
								loop {
									if by_err_use && buf[0] == 0 {
										break
									}
									if self.ret_ != Typ_::Void {
										as_ref__!(env.w).dunhao__(&mut ret)
									}
									ret.add__(s__(buf.as_ptr()));
									break;
								}
							}
							_ => {}
						}
					}
				}
				unsafe {
					for i in ret2 {
						let ret2 = Box::from_raw(i);
						for i in ret2.a_ {
							ret.add4__(i)
						}
					}
					drop(Box::from_raw(env2));
					let ret3 = Box::from_raw(ret3);
					if ret3.is_err() {
						match errret {
							callback_::NO_ERR_RETURN /*| callback_::NO_ERR_BREAK*/ => {}
							_ => return *ret3
						}
					}
				}
				return Ok(())
			}
			Ptr__::Ptr2 => {
				let q2 = Qv_::new2(Some(env.q.clone()));
				{
					let args2 = &mut as_mut_ref__!(q2.args_);
					{
						let mut idx = start;
						let mut args2__ = |argv:&Vec<Typ_>| {
							for t in argv {
								match t {
									Typ_::StringVal(i) => {
										args2.add__(i);
										if idx < args.len() {
											as_ref__!(env.w).dunhao__(args2)
										}
										continue
									}
									Typ_::Z => {
										while idx < args.len() {
											args2.add4__(args[idx].clone());
											idx += 1;
										}
										continue
									}
									_ => {}
								}
								if idx >= args.len() {
									return g_::err__('<')
								}
								let i = &args[idx];
								match t {
									Typ_::CharPtr => args2.add4__(i.clone()),
									_ => return no_typ__(t, ' ')
								}
								idx += 1;
							}
							ok__()
						};
						args2__(&self.argv_)?;
						args2__(&argv2)?;
						if idx < args.len() {
							return g_::err__('>')
						}
					}
				}
				//#[cfg(debug_assertions)] //居然致命？以下不得删
				if as_ref__!(env.w).dbg_.arg_ {
					as_ref__!(env.w).dbg_.arg__(&as_ref__!(q2.args_));
				}
				self.p_1_.unwrap()(&code_::Env_::new2(t__(q2), env))?;
				match self.ret_ {
					Typ_::Void => {}
					_ => return no_typ__(&self.ret_, 'r')
				}
			}
			Ptr__::Ptr3 => {
				{
					let args2__ = |argv:&Vec<Typ_>| {
						for t in argv {
							return no_typ__(t, ' ')
						}
						ok__()
					};
					args2__(&self.argv_)?;
					args2__(&argv2)?;
				}
				let mut ret2 = vec![];
				if let Err(err) = self.p_2_.unwrap()(&args.to_vec5__(start), &mut ret2) {
					return result2_::err__(err)
				}
				let mut ret = as_mut_ref__!(env.ret);
				let mut first = true;
				for i in ret2 {
					if first {
						first = false
					} else {
						as_ref__!(env.w).dunhao__(&mut ret)
					}
					ret.add__(i);
				}
			}
		}
		Ok(())
	}

	fn push__<I:IntoArg>(&self, i:I, args2:&mut Vec<usize>) {
		args2.extend_from_slice(&i.into_arg());
	}
	fn push2__<F>(&self, s:&str, args2:&mut Vec<usize>) -> Result2_
	where F: str::FromStr + IntoArg + fmt::Display
	{
		if let Ok(i) = s.parse::<F>() {
			self.push__(i, args2);
			ok__()
		} else {
			g_::err2__(s, 'n')
		}
	}
	fn push3__<F>(&self, s:&str, fargs2:&mut Vec<f64>, args2:&mut Vec<usize>) -> Result2_
	where F: str::FromStr + IntoArg + fmt::Display
	{
		if let Ok(i) = s.parse::<F>() {
			if fargs2.len() != 8 {
				unsafe {
					fargs2.push(mem::transmute_copy::<F, f64>(&i));
				}
			} else {
				self.push__(i, args2);
			}
			ok__()
		} else {
			g_::err2__(s, 'n')
		}
	}
}

unsafe impl Send for Item_ {}
unsafe impl Sync for Item_ {}

pub struct List_ {
	lib_:Library,
	a_:Vec<RI_>,
	lang_:String,
}

impl List_ {
	pub fn new(path:&[String]) -> Result<Self, String> {
		let mut lang_ = String::from("c");
		let mut err = String::new();
		for path in path {
			if path.starts_with('<') && path.ends_with('>') {
				lang_ = path[1..path.len() - 1].to_string();
				continue
			}
			match Library::new(path) {
				Ok(lib_) => return Ok(Self {lib_, a_:vec![], lang_}),
				Err(e) => {
					if !err.is_empty() {
						err.push('\n');
					}
					err.push_str(&e.to_string());
				}
			}
		}
		if !err.is_empty() {
			return Err(err)
		}
		Ok(Self {lib_:Library::from(limp_::Library::this()), a_:vec![], lang_})
	}
	
	pub fn add__(&mut self, vals:&[String], names:&[String]) -> Result<RI_, String> {
		unsafe {
			let args = if vals.len() > 1 {&vals[1]} else {""};
			let sym  = if vals.len() > 0 {&vals[0]} else {""};
			let sym = if sym.is_empty() {&names[0]} else {sym};
			let ri = match self.lang_.as_str() {
				"rust" => match self.lib_.get::<Ptr2_>(sym.as_bytes()) {
					Ok(p) => {
						let i = Item_::new2(*p, args)?;
						Rc_::new(i)
					}
					Err(e) => return Err(e.to_string())
				},
				"r" => match self.lib_.get::<Ptr3_>(sym.as_bytes()) {
					Ok(p) => {
						let i = Item_::new3(*p, args)?;
						Rc_::new(i)
					}
					Err(e) => return Err(e.to_string())
				},
				"c" => match self.lib_.get::<fn()>(sym.as_bytes()) {
					Ok(p) => {
						let i = Item_::new(*p.into_raw() as Ptr_, args)?;
						Rc_::new(i)
					}
					Err(e) => return Err(e.to_string())
				}
				_ => return Err(format!("不支持 {} 语言", self.lang_))
			};
			self.a_.push(ri.clone());
			Ok(ri)
		}
	}
}
