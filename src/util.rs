
pub fn get_type_of<T>(_: &T) -> String {
	  unsafe {
		    ::std::intrinsics::type_name::<T>()
	  }.to_owned()
}

#[macro_export]
macro_rules! dump(
	  ($($a:expr),*) => {
		    println!(concat!("[", file!(), ":", line!(), "] ", $(stringify!($a), ": {} = {:#?}; "),*), $($crate::get_type_of(&$a), $a),*);
	  }
);
