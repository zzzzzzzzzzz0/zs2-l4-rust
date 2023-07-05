use zhscript2::{u_::*};

#[derive(PartialEq, Clone, Debug)]
pub enum Typ_ {
	Void, Int, UInt, Long, ULong, Float, Char, CharPtr, CharAddr, Addr, NBool,
	Unsigned, Ptr, Val,
	IntVal(i64), UIntVal(u64), StringVal(String),
	Buf, BufSiz, Addrret, Errret, Z, Ret, VecAddr,
	CallBack, Callback(String),
	No, Err
}

pub fn no_typ__(t:&Typ_, sty:char) -> Result2_ {
	result2_::err__(match sty {
		'r' => format!("返回类型 {:?} 不支持", t),
		' ' => format!("参数类型 {:?} 未实现", t),
		_ => panic!()
	})
}

pub fn typ__(c:u8) -> Typ_ {
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
		b'R' => Typ_::Ret,
		b'V' => Typ_::VecAddr,
		b'$' => Typ_::CallBack,
		b'-' => Typ_::No,
		_ => Typ_::Err,
	}
}

pub fn typ3__(t2:&mut [Typ_], s2:&mut String) -> Typ_ {
	let mut ret = Typ_::Err;
	if Typ_::No == t2[1] {
		if t2[0] == Typ_::Unsigned {
			ret = Typ_::UInt
		} else {
			ret = t2[0].clone();
		}
	} else {
		if t2[0] == Typ_::Char && t2[1] == Typ_::Ptr {
			ret = Typ_::CharPtr
		} else if t2[0] == Typ_::Int && t2[1] == Typ_::Unsigned
		       || t2[1] == Typ_::Int && t2[0] == Typ_::Unsigned {
			ret = Typ_::UInt
		} else if t2[0] == Typ_::Long && t2[1] == Typ_::Unsigned
		       || t2[1] == Typ_::Long && t2[0] == Typ_::Unsigned {
			ret = Typ_::ULong
		} else if (t2[0] == Typ_::Int || t2[0] == Typ_::Long) && t2[1] == Typ_::Val {
			if let Ok(i) = s2.parse::<i64>() {
				ret = Typ_::IntVal(i)
			}
		} else if t2[0] == Typ_::Unsigned && t2[1] == Typ_::Val {
			if let Ok(i) = s2.parse::<u64>() {
				ret = Typ_::UIntVal(i)
			}
		} else if t2[0] == Typ_::Ptr && t2[1] == Typ_::Val {
			ret = Typ_::StringVal(s2.to_string())
		} else if t2[0] == Typ_::CallBack && t2[1] == Typ_::Val {
			ret = Typ_::Callback(s2.to_string())
		} else if t2[0] == Typ_::Char && t2[1] == Typ_::Addr {
			ret = Typ_::CharAddr
		}
		t2[1] = Typ_::No;
	}
	t2[0] = Typ_::Void;
	s2.clear();
	ret
}
	
pub fn pargs__(args:&str, ret_:&mut Typ_, argv_:&mut Vec<Typ_>) -> Result<(), String> {
	let mut t2 = [Typ_::Void, Typ_::No];
	let mut i2 = 0;
	
	let mut first = true;
	let mut idx = 0;
	let mut s2 = String::new();
	let args = args.to_string() + "-";
	let cs = args.as_bytes();
	'l1: while idx < cs.len() {
		let c = cs[idx];
		let t = typ__(c);
		if Typ_::Err == t {
			return Err(format!("{} 非参数类型符", c as char))
		}
		if first {
			match t {
				Typ_::No => {
					*ret_ = typ3__(&mut t2, &mut s2);
					i2 = 0;
					first = false
				}
				_ => typ2__(t, &mut t2, &mut i2)?
			}
		} else {
			match t {
				Typ_::No => {
					argv_.push(typ3__(&mut t2, &mut s2));
					i2 = 0;
				}
				_ => typ2__(t, &mut t2, &mut i2)?
			}
		}
		if t2[1] == Typ_::Val {
			let mut c = || {
				idx += 1;
				if idx >= cs.len() {
					return Err(format!("{}位置意外", idx))
				}
				Ok(cs[idx])
			};
			match t2[0] {
				Typ_::Int | Typ_::Long | Typ_::Unsigned => {
					let mut hao = t2[0] != Typ_::Unsigned;
					loop {
						match c() {
							Err(e) => return Err(e),
							Ok(c) => {
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
					}
				}
				Typ_::Ptr => {
					let mut hao = true;
					let mut hao2 = 0;
					loop {
						match c() {
							Err(e) => return Err(e),
							Ok(c) => {
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
					}
				}
				Typ_::CallBack => {
					loop {
						match c() {
							Err(e) => return Err(e),
							Ok(c) => {
								match c {
									b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => s2.push(c as char),
									_ => continue 'l1
								}
							}
						}
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
	pargs__(args, &mut ret, argv2)
}
	
pub fn typ2__(t:Typ_, t2:&mut [Typ_], i2:&mut usize) -> Result<(), String> {
	if *i2 >= t2.len() {
		return Err(format!("{}>={} len 错误参数格式", *i2, t2.len()))
	}
	t2[*i2] = t;
	*i2 += 1;
	Ok(())
}
