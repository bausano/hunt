use std::error::Error;

pub use crate::conf;

/// This will work just fine for us, there isn't need for custom error.
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub type Vector2 = cgmath::Vector2<f32>;
