mod color;
mod point;
mod triangle;

pub use color::*;
pub use point::*;
pub use triangle::*;

pub type Mesh = Vec<triangle::Triangle>;
