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

#![allow(non_upper_case_globals)]

#[macro_use(bitflags)]
extern crate bitflags;
extern crate libc;
extern crate xkbcommon_sys as ffi;

#[cfg(feature = "x11")]
extern crate xcb;

#[macro_use]
mod base;
pub use self::base::*;

mod keysym;
pub use keysym::Keysym;

pub mod context;
pub use context::Context;

pub mod keymap;
pub use keymap::Keymap;

pub mod state;
pub use state::State;

pub mod name;
pub mod key;
pub mod compose;

#[cfg(feature = "x11")]
pub mod x11;
