mod euler;
mod float;
mod vector3;

pub use euler::*;
pub use float::*;
pub use vector3::*;

pub trait IsFinite {
    fn is_finite(&self) -> bool;
}
