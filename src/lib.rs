mod euler;
mod float;
mod vector3;

pub use euler::*;
pub use float::*;
pub use vector3::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
