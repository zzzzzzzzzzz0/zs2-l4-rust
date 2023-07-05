use zhscript2::{u_::*, };

pub fn err__(typ:char) -> Result2_ {
	result2_::err2__(match typ {
		'<' => "参数不足",
		'>' => "参数超量",
		_ => panic!()
	})
}
pub fn err2__(s:&str, typ:char) -> Result2_ {
	result2_::err__(match typ {
		'n' => format!("{} 非数字", s),
		_ => panic!()
	})
}
