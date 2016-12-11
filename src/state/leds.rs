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

use std::ffi::{CString, CStr};

use ffi::*;
use {State, LedIndex};

#[derive(Debug)]
pub struct Leds<'a>(pub &'a State);

impl<'a> Leds<'a> {
	pub fn active<P: Into<Parameter>>(&self, parameter: P) -> bool {
		unsafe {
			match parameter.into() {
				Parameter::Name(name) =>
					xkb_state_led_name_is_active(self.0.as_ptr(), name.as_ptr()) != 0,

				Parameter::Index(index) =>
					xkb_state_led_index_is_active(self.0.as_ptr(), index.into()) != 0,
			}
		}
	}
}

#[derive(Debug)]
pub enum Parameter {
	Name(CString),
	Index(LedIndex),
}

impl<'a> From<&'a [u8]> for Parameter {
	fn from(value: &[u8]) -> Parameter {
		unsafe {
			Parameter::Name(CStr::from_ptr(value.as_ptr() as *const _).to_owned())
		}
	}
}

impl<'a> From<&'a str> for Parameter {
	fn from(value: &str) -> Parameter {
		Parameter::Name(CString::new(value).unwrap())
	}
}

impl<'a> From<String> for Parameter {
	fn from(value: String) -> Parameter {
		Parameter::Name(CString::new(value).unwrap())
	}
}

impl From<LedIndex> for Parameter {
	fn from(value: LedIndex) -> Parameter {
		Parameter::Index(value)
	}
}

impl From<xkb_led_index_t> for Parameter {
	fn from(value: xkb_led_index_t) -> Parameter {
		Parameter::Index(value.into())
	}
}
