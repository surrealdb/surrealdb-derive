// cargo expand --example key

use surrealdb_derive::Key;

mod err {
	pub type Error = ();
}

mod sql {
	pub mod serde {
		pub fn beg_internal_serialization() {}
		pub fn end_internal_serialization() {}
	}
}

#[derive(Key)]
pub struct NsOwned {
	__: u8,
	_a: u8,
	_b: u8,
	_c: u8,
	pub ns: String,
}

/// WIP: Support for borrowed keys.
#[derive(Key)]
pub struct NsBorrowed<'a>
// Extra where bound for testing macro.
where
	'a: 'static,
{
	__: u8,
	_a: u8,
	_b: u8,
	_c: u8,
	pub ns: &'a str,
}
