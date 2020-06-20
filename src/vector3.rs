use super::euler::Euler;
use super::IsFinite;

use alga::general::RealField;
use nalgebra::{Scalar, Vector3};

pub trait SwapYZ {
    fn swap_yz(&self) -> Self;
}

impl<N: Scalar + Copy> SwapYZ for Vector3<N> {
    fn swap_yz(&self) -> Self {
        Vector3::new(self.x, self.z, self.y)
    }
}

impl<N: Scalar + RealField> IsFinite for Vector3<N> {
    fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
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
        if self.x == 0f32 && self.z == 0f32 {
            let pitch = {
                if self.y > 0f32 {
                    270f32
                } else {
                    90f32
                }
            };
            Euler::new(pitch, 0f32, 0f32).normalize()
        } else {
            // magnitude_2d ?
            let mut pitch = (-self.y).atan2(self.magnitude_xz()).to_degrees();
            if pitch < 0f32 {
                pitch += 360f32;
            }

            let mut yaw = self.z.atan2(self.x).to_degrees();
            if yaw < 0f32 {
                yaw += 360f32;
            }

            Euler::new(pitch, yaw, 0f32).normalize()
        }
    }

    fn euler_angles_with_up(&self, up: &Vector3<f32>) -> Euler<f32> {
        let left = up.cross(self).normalize();

        let distxz = self.magnitude_xz();
        let pitch = (-self.y).atan2(distxz).to_degrees();

        if distxz > 0.001f32 {
            let up_y = (self.x * left.z) - (self.z * left.x);
            Euler::new(
                pitch,
                self.z.atan2(self.x).to_degrees(),
                left.y.atan2(up_y).to_degrees(),
            )
            .normalize()
        } else {
            Euler::new(pitch, (-left.x).atan2(left.z).to_degrees(), 0f32).normalize()
        }
    }
}

impl VectorAngles<f64> for Vector3<f64> {
    fn euler_angles(&self) -> Euler<f64> {
        if self.x == 0f64 && self.z == 0f64 {
            let pitch = {
                if self.y > 0f64 {
                    270f64
                } else {
                    90f64
                }
            };
            Euler::new(pitch, 0f64, 0f64).normalize()
        } else {
            // magnitude_2d ?
            let mut pitch = (-self.y).atan2(self.magnitude_xz()).to_degrees();
            if pitch < 0f64 {
                pitch += 360f64;
            }

            let mut yaw = self.z.atan2(self.x).to_degrees();
            if yaw < 0f64 {
                yaw += 360f64;
            }

            Euler::new(pitch, yaw, 0f64).normalize()
        }
    }

    fn euler_angles_with_up(&self, up: &Vector3<f64>) -> Euler<f64> {
        let left = up.cross(self).normalize();

        let distxz = self.magnitude_xz();
        let pitch = (-self.y).atan2(distxz).to_degrees();

        if distxz > 0.001f64 {
            let up_y = (self.x * left.z) - (self.z * left.x);
            Euler::new(
                pitch,
                self.z.atan2(self.x).to_degrees(),
                left.y.atan2(up_y).to_degrees(),
            )
            .normalize()
        } else {
            Euler::new(pitch, (-left.x).atan2(left.z).to_degrees(), 0f64).normalize()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_swap() {
        let a = Vector3::new(0, 1, 2);
        assert_eq!(a.swap_yz(), Vector3::new(0, 2, 1));
        assert_eq!(a.swap_yz(), Vector3::new(0, 2, 1));
        assert_eq!(a.swap_yz().swap_yz(), a);
    }

    #[test]
    fn test_magnitude2d() {
        let a = Vector3::new(1f32, 0f32, 0f32);
        assert_approx_eq!(a.norm_xz(), 1f32);
        let b = Vector3::new(1f32, 2f32, 3f32);
        assert_approx_eq!(b.norm_xz_squared(), 10f32);
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
