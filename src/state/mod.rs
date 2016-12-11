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

mod state;
pub use self::state::State;

mod key;
pub use self::key::Key;

mod serialize;
pub use self::serialize::Serialize;

mod update;
pub use self::update::Update;

mod mods;
pub use self::mods::Mods;

mod layouts;
pub use self::layouts::Layouts;

mod leds;
pub use self::leds::Leds;

pub mod component;
pub use self::component::Components;

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Consumed {
	Xkb,
	Gtk,
}
