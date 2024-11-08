use zhscript2::{u_::*, as_mut_ref__, cfg_if};
use super::*;

cfg_if! {
	if #[cfg(not(feature = "no_dl"))] {

struct U11_ {}
impl pars_::U11_ for U11_ {
	fn u11__(&self, kw:keyword_::RI_, kws:&keyword_::List_, _:& code_::List_) -> code_::OI_ {
		if kw.s_ == dl_::NAME_ {
			return code_::oi__(dl_::Item_::new(kw))
		}
		if kw.s_ == func_::NAME_ {
			return code_::oi__(func_::Item_::new(kw, kws))
		}
		if kw.s_ == func_next_::NAME_ {
			return code_::oi__(func_next_::Item_::new(kw, kws))
		}
		if kw.s_ == call_::NAME_ {
			return code_::oi__(call_::Item_::new(kw))
		}
		None
	}
}

}}

pub fn w__() -> world_::T_ {
	let w = t__(World_::new());
	cfg_if! {
		if #[cfg(not(feature = "no_dl"))] {
			{
				let kws = &mut as_mut_ref__!(w).kws_;
				kws.add__ (  dl_::NAME_, keyword_::Id_::U11);
				kws.add3__(func_::NAME_, keyword_::Id_::U11, keyword_::Grp_ {set_:true, ..Default::default()});
				kws.add3__(func_next_::NAME_, keyword_::Id_::U11, keyword_::Grp_ {set_:true, ..Default::default()});
				kws.add__ (call_::NAME_, keyword_::Id_::U11);

				kws.add__("为", keyword_::Id_::Dunhao);
			}
			as_mut_ref__!(w).pars_.u11_ = Some(Rc_::new(U11_ {}));
		}
	}
	w
}
