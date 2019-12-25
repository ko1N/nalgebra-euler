use super::float::NormalizeAngle;
use super::euler::Euler;
use alga::general::RealField;
use nalgebra::{Scalar, Vector3};

pub trait SwapYZ {
    fn swap_yz(&self) -> Self;
}

impl<N: Scalar> SwapYZ for Vector3<N> {
    fn swap_yz(&self) -> Self {
        Vector3::new(self.x, self.z, self.y)
    }
}

pub trait Magnitude2D<N: Scalar + RealField> {
    fn norm_xz_squared(&self) -> N::RealField;

    fn norm_xz(&self) -> N::RealField {
        self.norm_xz_squared().sqrt()
    }

    fn magnitude_xz_squared(&self) -> N::RealField {
        self.norm_xz_squared()
    }

    fn magnitude_xz(&self) -> N::RealField {
        self.norm_xz()
    }
}

impl<N: Scalar + RealField> Magnitude2D<N> for Vector3<N> {
    fn norm_xz_squared(&self) -> N::RealField {
        (self.x * self.x) + (self.z * self.z)
    }
}

pub trait VectorAngles<N: Scalar> {
    fn euler_angles(&self) -> Euler<N>;
    fn euler_angles_with_up(&self, up: &Vector3<N>) -> Euler<N>;
}

impl VectorAngles<f32> for Vector3<f32> {
    fn euler_angles(&self) -> Euler<f32> {
        if self.x == 0.0 && self.z == 0.0 {
            let pitch = {
                if self.y > 0.0 {
                    270.0
                } else {
                    90.0
                }
            };
            Euler::new(pitch.normalize_angle(), 0.0, 0.0)
        } else {
            // magnitude_2d ?
            let mut pitch = (-self.y).atan2(self.magnitude_xz()).to_degrees();
            if pitch < 0.0 {
                pitch += 360.0;
            }

            let mut yaw = self.z.atan2(self.x).to_degrees();
            if yaw < 0.0 {
                yaw += 360.0;
            }

            Euler::new(pitch.normalize_angle(), yaw.normalize_angle(), 0.0)
        }
    }

    fn euler_angles_with_up(&self, up: &Vector3<f32>) -> Euler<f32> {
        let left = up.cross(self);
        left.normalize();

        let distxz = self.magnitude_xz();
        let pitch = (-self.y).atan2(distxz).to_degrees();

        if distxz > 0.001 {
            let up_y = (self.x * left.z) - (self.z * left.x);
            Euler::new(
                pitch.normalize_angle(),
                self.z.atan2(self.x).to_degrees().normalize_angle(),
                left.y.atan2(up_y).to_degrees().normalize_angle(),
            )
        } else {
            Euler::new(
                pitch.normalize_angle(),
                (-left.x).atan2(left.z).to_degrees().normalize_angle(),
                0.0,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap() {
        let a = Vector3::new(0, 1, 2);
        assert_eq!(a.swap_yz(), Vector3::new(0, 2, 1));
        assert_eq!(a.swap_yz(), Vector3::new(0, 2, 1));
        assert_eq!(a.swap_yz().swap_yz(), a);
    }

    #[test]
    fn test_magnitude2d() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(a.norm_xz(), 1.0);
        let b = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(b.norm_xz_squared(), 10.0);
    }

    #[test]
    fn test_vector_angles() {
        let a = Vector3::new(0.0, 5.0, 5.0);
        assert_eq!(a.euler_angles(), Euler::new(-45.0, 90.0, 0.0));
        let b = Vector3::new(0.0, 10.0, 10.0);
        assert_eq!(b.euler_angles(), Euler::new(-45.0, 90.0, 0.0));
        let c = Vector3::new(1.0, 1.0, 0.0);
        assert_eq!(c.euler_angles(), Euler::new(-45.0, 0.0, 0.0));
    }
}
