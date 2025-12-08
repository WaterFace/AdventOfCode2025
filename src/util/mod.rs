pub mod binsearch;
pub mod dir;
pub mod ext;
pub mod grid;
pub mod math;
pub mod parse_ints;
pub mod sparse_grid;
pub mod stats;
pub mod vec2;
pub mod vec3;

pub use binsearch::*;
pub use dir::Dir;
pub use ext::*;
pub use grid::Grid;
pub use math::*;
pub use parse_ints::*;
pub use sparse_grid::SparseGrid;
pub use vec2::{Vec2, vec2};
pub use vec3::{Vec3, vec3};

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}
