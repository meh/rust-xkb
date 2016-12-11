//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

mod table;
pub use self::table::Table;

pub mod compile;

pub mod state;
pub use self::state::State;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Status {
	Nothing,
	Composing,
	Composed,
	Cancelled,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Result {
	Ignored,
	Accepted,
}

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Format {
	TextV1,
}
