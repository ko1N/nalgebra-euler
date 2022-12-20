mod euler;
mod float;
mod vector3;

pub use euler::*;
pub use float::*;
pub use vector3::*;

pub trait IsFinite {
    fn is_finite(&self) -> bool;
}

pub mod prelude {
    pub mod v1 {
        pub use crate::euler::*;
        pub use crate::float::*;
        pub use crate::vector3::*;
        pub use crate::IsFinite;

        // external libraries
        pub use ::nalgebra;
    }
}

// external libraries
pub use ::nalgebra;
