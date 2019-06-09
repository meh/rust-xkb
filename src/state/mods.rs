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

use libc::c_char;
use ffi::*;
use crate::{State, ModIndex};
use crate::state::Components;

#[derive(Debug)]
pub struct Mods<'a>(pub &'a State);

impl<'a> Mods<'a> {
	pub fn active<P: Into<Parameter>>(&self, parameter: P, component: Components) -> bool {
		unsafe {
			match parameter.into() {
				Parameter::Name(name) =>
					xkb_state_mod_name_is_active(self.0.as_ptr(), name.as_ptr(), component.bits()) != 0,

				Parameter::Index(index) =>
					xkb_state_mod_index_is_active(self.0.as_ptr(), index.into(), component.bits()) != 0,
			}
		}
	}
}

#[derive(Debug)]
pub enum Parameter {
	Name(CString),
	Index(ModIndex),
}

impl From<*const c_char> for Parameter {
	fn from(value: *const c_char) -> Parameter {
		unsafe {
			Parameter::Name(CStr::from_ptr(value).to_owned())
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

impl From<ModIndex> for Parameter {
	fn from(value: ModIndex) -> Parameter {
		Parameter::Index(value)
	}
}

impl From<xkb_mod_index_t> for Parameter {
	fn from(value: xkb_mod_index_t) -> Parameter {
		Parameter::Index(value.into())
	}
}
