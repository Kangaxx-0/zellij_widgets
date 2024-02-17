pub use self::{attribute::Attribute, color::Color, colored::Colored};

pub use super::{ansi::SetAttribute, parse_next_u8};

mod attribute;
mod color;
mod colored;
