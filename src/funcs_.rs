use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use std::{mem, ffi::CStr, str, fmt};
use libloading::{Library, os::unix as limp_};

mod call_;
use call_::{IntoArg, Ptr_};

#[derive(PartialEq, Clone, Debug)]
pub enum Typ_ {
	Void, Int, UInt, Long, ULong, Float, Char, CharPtr, CharAddr, Addr, NBool,
	Unsigned, Ptr, Val,
	IntVal(i64), UIntVal(u64), StringVal(String),
	Buf, BufSiz, Addrret, Errret, Z,
	No, Err
}

pub type RI_ = Rc_<Item_>;

pub type Ptr2_ = fn(env:&code_::Env_) -> Result2_;

#[derive(Debug)]
pub enum Ptr__ {
	Ptr,
	Ptr2,
}

pub struct Item_ {
	p_:Ptr__,
	p_0_:Option<Ptr_>,
	p_1_:Option<Ptr2_>,
	argv_:Vec<Typ_>,
	ret_:Typ_,
}

impl Item_ {
	fn new(p:Ptr_, args:&str) -> Result<Self, String> {
		Self::new3(Ptr__::Ptr, Some(p), None, args)
	}
	fn new2(p:Ptr2_, args:&str) -> Result<Self, String> {
		Self::new3(Ptr__::Ptr2, None, Some(p), args)
	}
	fn new3(p_:Ptr__, p_0_:Option<Ptr_>, p_1_:Option<Ptr2_>, args:&str) -> Result<Self, String> {
		let mut argv_ = vec![];
		let mut ret_ = Typ_::Void;
		Self::pargs__(args, &mut ret_, &mut argv_)?;
		Ok(Self {p_, p_0_, p_1_, argv_, ret_})
	}
	
	fn typ__(c:u8) -> Typ_ {
		match c {
			b'i' => Typ_::Int,
			b'l' => Typ_::Long,
			b'f' => Typ_::Float,
			b'c' => Typ_::Char,
			b'u' => Typ_::Unsigned,
			b'*' => Typ_::Ptr,
			b'&' => Typ_::Addr,
			b'n' => Typ_::NBool,
			b':' => Typ_::Val,
			b'B' => Typ_::Buf,
			b'S' => Typ_::BufSiz,
			b'A' => Typ_::Addrret,
			b'E' => Typ_::Errret,
			b'Z' => Typ_::Z,
			b'-' => Typ_::No,
			_ => Typ_::Err,
		}
	}
	
	fn typ3__(t2:&mut [Typ_], s2:&mut String) -> Typ_ {
		let mut ret = Typ_::Err;
		if Typ_::No == t2[1] {
			ret = t2[0].clone();
		} else {
			if t2[0] == Typ_::Char && t2[1] == Typ_::Ptr {
				ret = Typ_::CharPtr
			} else if t2[1] == Typ_::Int && t2[0] == Typ_::Unsigned {
				ret = Typ_::UInt
			} else if t2[0] == Typ_::Long && t2[1] == Typ_::Unsigned {
				ret = Typ_::ULong
			} else if t2[0] == Typ_::Int && t2[1] == Typ_::Val {
				if let Ok(i) = s2.parse::<i64>() {
					ret = Typ_::IntVal(i)
				}
			} else if t2[0] == Typ_::Unsigned && t2[1] == Typ_::Val {
				if let Ok(i) = s2.parse::<u64>() {
					ret = Typ_::UIntVal(i)
				}
			} else if t2[0] == Typ_::Ptr && t2[1] == Typ_::Val {
				ret = Typ_::StringVal(s2.to_string())
			} else if t2[0] == Typ_::Char && t2[1] == Typ_::Addr {
				ret = Typ_::CharAddr
			}
			t2[1] = Typ_::No;
		}
		t2[0] = Typ_::Void;
		s2.clear();
		ret
	}
	
	pub fn call__(&self, argv2:&Vec<Typ_>, args:&result_::List_, start:usize, env:&code_::Env_) -> Result2_ {
		match self.p_ {
			Ptr__::Ptr => {
				let args = args.to_vec5__(start);
				let mut args2: Vec<usize> = vec![];
				let mut fargs2: Vec<f64> = vec![];
				let mut args_0 = vec![];
				let mut buf = vec![0i8; 512]; //1953
				let mut errret = 0;
				let errret2:*mut i32 = &mut errret;
				{
					let mut idx = 0;
					let mut push_s__ = |i:&str, args2:&mut Vec<usize>| {
						self.push__(if i.ends_with("\0") {
							i.as_ptr()
						} else {
							let s = i.to_string() + "\0";
							args_0.push(s);
							args_0[args_0.len() - 1].as_ptr()
						}, args2)
					};
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
								_ => {}
							}
							if idx >= args.len() {
								return result2_::err2__("参数不足")
							}
							let i = &args[idx];
							match t {
								Typ_::Int => {self.push2__::<i32>(i, &mut args2)?}
								Typ_::UInt => {self.push2__::<u32>(i, &mut args2)?}
								Typ_::Long => {self.push2__::<i64>(i, &mut args2)?}
								Typ_::ULong | Typ_::Addr => {self.push2__::<u64>(i, &mut args2)?}
								Typ_::Float => {self.push3__::<f64>(i, &mut fargs2, &mut args2)?}
								Typ_::Char => self.push__(if i.is_empty() {0} else {i.as_bytes()[0]}, &mut args2),
								Typ_::CharPtr => push_s__(i, &mut args2),
								_ => return result2_::err__(format!("参数类型 {:?} 未实现", t))
							}
							idx += 1;
						}
						ok__()
					};
					args2__(&self.argv_)?;
					args2__(&argv2)?;
					if idx < args.len() {
						return result2_::err2__("参数超量")
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
					Typ_::CharPtr => ret.add__(self.s__(ret_low as *const i8)),
					Typ_::NBool => ret.add__(if ret_low == 0 {"1"} else {"0"}),
					_ => return result2_::err__(format!("返回类型 {:?} 不支持", self.ret_))
				}
				for t in &self.argv_ {
					match t {
						Typ_::Buf | Typ_::BufSiz => ret.add__(self.s__(buf.as_ptr())),
						Typ_::Errret => {
							if errret != 0 {
								return result2_::err__(format!("{} 错误码{}", self.s__(buf.as_ptr()), errret))
							}
						}
						_ => {}
					}
				}
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
										continue
									}
									Typ_::Z => {
										let len = args.len();
										while idx < len {
											args2.push(args[idx].clone());
											idx += 1;
										}
										continue
									}
									_ => {}
								}
								if idx >= args.len() {
									return result2_::err2__("参数不足")
								}
								let i = &args[idx];
								match t {
									Typ_::CharPtr => args2.push(i.clone()),
									_ => return result2_::err__(format!("参数类型 {:?} 未实现", t))
								}
								idx += 1;
							}
							ok__()
						};
						args2__(&self.argv_)?;
						args2__(&argv2)?;
						if idx < args.len() {
							return result2_::err2__("参数超量")
						}
					}
				}
				if as_ref__!(env.w).dbg_.arg_ {
					as_ref__!(env.w).dbg_.arg__(&as_ref__!(q2.args_));
				}
				self.p_1_.unwrap()(&code_::Env_::new2(t__(q2), env))?;
				match self.ret_ {
					Typ_::Void => {}
					_ => return result2_::err__(format!("返回类型 {:?} 不支持", self.ret_))
				}
			}
		}
		Ok(())
	}
	
	fn s__<'a>(&self, i:*const i8) -> &'a str {
		if i as usize == 0 {
			"NULL"
		} else {unsafe {
			CStr::from_ptr(i).to_str().unwrap()
		}}
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
			result2_::err__(format!("{} 非数字", s))
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
			result2_::err__(format!("{} 非数字", s))
		}
	}
	
	fn pargs__(args:&str, ret_:&mut Typ_, argv_:&mut Vec<Typ_>) -> Result<(), String> {
		let mut t2 = [Typ_::Void, Typ_::No];
		let mut i2 = 0;
		
		let mut first = true;
		let mut idx = 0;
		let mut s2 = String::new();
		let args = args.to_string() + "-";
		let cs = args.as_bytes();
		'l1: while idx < cs.len() {
			let c = cs[idx];
			let t = Self::typ__(c);
			if Typ_::Err == t {
				return Err(format!("{} 非参数类型符", c as char))
			}
			if first {
				match t {
					Typ_::No => {
						*ret_ = Self::typ3__(&mut t2, &mut s2);
						i2 = 0;
						first = false
					}
					_ => Self::typ2__(t, &mut t2, &mut i2)?
				}
			} else {
				match t {
					Typ_::No => {
						argv_.push(Self::typ3__(&mut t2, &mut s2));
						i2 = 0;
					}
					_ => Self::typ2__(t, &mut t2, &mut i2)?
				}
			}
			if t2[1] == Typ_::Val {
				match t2[0] {
					Typ_::Int | Typ_::Unsigned => {
						let mut hao = t2[0] == Typ_::Int;
						loop {
							idx += 1;
							if idx >= cs.len() {
								return Err(format!("{}位置意外", idx))
							}
							let c = cs[idx];
							if hao {
								hao = false;
								match c {
									b'-' | b'+' => {
										s2.push(c as char);
										continue
									}
									_ => {}
								}
							}
							match c {
								b'0'..=b'9' => s2.push(c as char),
								_ => continue 'l1
							}
						}
					}
					Typ_::Ptr => {
						let mut hao = true;
						let mut hao2 = 0;
						loop {
							idx += 1;
							if idx >= cs.len() {
								return Err(format!("{}位置意外", idx))
							}
							let c = cs[idx];
							if hao {
								hao = false;
								hao2 = c;
								continue
							}
							if c == hao2 {
								break
							}
							s2.push(c as char)
						}
					}
					_ => {}
				}
			}
			idx += 1
		}
		Ok(())
	}
	pub fn pargs2__(args:&str, argv2:&mut Vec<Typ_>) -> Result<(), String> {
		let mut ret = Typ_::No;
		Self::pargs__(args, &mut ret, argv2)
	}
	
	fn typ2__(t:Typ_, t2:&mut [Typ_], i2:&mut usize) -> Result<(), String> {
		if *i2 >= t2.len() {
			return Err(format!("{}>={} len 错误参数格式", *i2, t2.len()))
		}
		t2[*i2] = t;
		*i2 += 1;
		Ok(())
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
	
	pub fn add__(&mut self, vals:&[String]) -> Result<RI_, String> {
		unsafe {
			let ri = match self.lang_.as_str() {
				"rust" => match self.lib_.get::<Ptr2_>(vals[0].as_bytes()) {
					Ok(p) => {
						let i = Item_::new2(*p, &vals[1])?;
						Rc_::new(i)
					}
					Err(e) => return Err(e.to_string())
				},
				"c" => match self.lib_.get::<fn()>(vals[0].as_bytes()) {
					Ok(p) => {
						let i = Item_::new(*p.into_raw() as Ptr_, &vals[1])?;
						Rc_::new(i)
					}
					Err(e) => return Err(e.to_string())
				}
				_ => return Err(format!("不支持 {}", self.lang_))
			};
			self.a_.push(ri.clone());
			Ok(ri)
		}
	}
}
