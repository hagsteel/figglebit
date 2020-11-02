mod font;
mod render;
pub(crate) mod smushing;

pub use font::{parse, Font};
pub use font::character::Char;
pub use render::{init, cleanup, Renderer};
