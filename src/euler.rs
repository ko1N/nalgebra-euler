use super::float::NormalizeAngle;
use super::vector3::{SwapYZ, VectorAngles};
use super::IsFinite;

use alga::general::RealField;
use nalgebra::{Scalar, Vector3};

pub trait AngleVectors<N: Scalar> {
    /// Returns the forward vector for this angle
    fn forward(&self) -> Vector3<N>;

    /// Returns the right vector for this angle
    fn right(&self) -> Vector3<N>;

    /// Returns the up vector for this angle
    fn up(&self) -> Vector3<N>;

    /// Returns forward, right and up vector (in this order) for this angle
    fn vectors(&self) -> (Vector3<N>, Vector3<N>, Vector3<N>);
}

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Euler<N: Scalar> {
    pub p: N,
    pub y: N,
    pub r: N,
}

impl<N: Scalar> Euler<N> {
    #[inline]
    pub fn new(p: N, y: N, r: N) -> Self {
        Self { p, y, r }
    }
}

impl<N: Scalar> From<(N, N, N)> for Euler<N> {
    #[inline]
    fn from(value: (N, N, N)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<N: Scalar + RealField> IsFinite for Euler<N> {
    #[inline]
    fn is_finite(&self) -> bool {
        self.p.is_finite() && self.y.is_finite() && self.r.is_finite()
    }
}

#[macro_export]
macro_rules! impl_euler {
    ($typ:tt) => {
        impl Euler<$typ> {
            // Normalizes this angle
            pub fn normalize(&mut self) {
                self.p = self.p.normalize_angle();
                self.y = self.y.normalize_angle();
                self.r = self.r.normalize_angle();
            }

            // Creates a normalized copy of this angle
            pub fn normalized(&self) -> Self {
                Euler::new(
                    self.p.normalize_angle(),
                    self.y.normalize_angle(),
                    self.r.normalize_angle(),
                )
            }

            pub fn lerp(&self, other: &Euler<$typ>, amount: $typ) -> Euler<$typ> {
                let v1 = self.forward().normalize();
                let v2 = other.forward().normalize();

                let mut delta = v2 - v1;
                let mag = delta.magnitude();
                delta /= amount;
                let new_mag = delta.magnitude();

                if new_mag < std::$typ::EPSILON || new_mag > mag {
                    // prevent overshoot
                    *other
                } else {
                    let t = v1 + delta;
                    let out = t.euler_angles().normalized();
                    if !out.is_finite() {
                        *other
                    } else {
                        out
                    }
                }
            }
        }

        impl AngleVectors<$typ> for Euler<$typ> {
            fn forward(&self) -> Vector3<$typ> {
                let p = self.p.to_radians().sin_cos();
                let y = self.y.to_radians().sin_cos();

                Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
                    .swap_yz()
                    .normalize()
            }

            fn right(&self) -> Vector3<$typ> {
                let p = self.p.to_radians().sin_cos();
                let y = self.y.to_radians().sin_cos();
                let r = self.r.to_radians().sin_cos();

                Vector3::new(
                    -1.0 * r.0 * p.0 * y.1 + -1.0 * r.1 * -y.0,
                    -1.0 * r.0 * p.0 * y.0 + -1.0 * r.1 * y.1,
                    -1.0 * r.0 * p.1,
                )
                .swap_yz()
                .normalize()
            }

            fn up(&self) -> Vector3<$typ> {
                let p = self.p.to_radians().sin_cos();
                let y = self.y.to_radians().sin_cos();
                let r = self.r.to_radians().sin_cos();

                Vector3::new(
                    r.1 * p.0 * y.1 + -r.0 * -y.0,
                    r.1 * p.0 * y.0 + -r.0 * y.1,
                    r.1 * p.1,
                )
                .swap_yz()
                .normalize()
            }

            fn vectors(&self) -> (Vector3<$typ>, Vector3<$typ>, Vector3<$typ>) {
                let p = self.p.to_radians().sin_cos();
                let y = self.y.to_radians().sin_cos();
                let r = self.r.to_radians().sin_cos();

                let forward = Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
                    .swap_yz()
                    .normalize();

                let right = Vector3::new(
                    -1.0 * r.0 * p.0 * y.1 + -1.0 * r.1 * -y.0,
                    -1.0 * r.0 * p.0 * y.0 + -1.0 * r.1 * y.1,
                    -1.0 * r.0 * p.1,
                )
                .swap_yz()
                .normalize();

                let up = Vector3::new(
                    r.1 * p.0 * y.1 + -r.0 * -y.0,
                    r.1 * p.0 * y.0 + -r.0 * y.1,
                    r.1 * p.1,
                )
                .swap_yz()
                .normalize();

                (forward, right, up)
            }
        }
    };
}

impl_euler!(f32);
impl_euler!(f64);

// TODO: create to_radians and sin_cos impl on N ?

// TODO: use ClosedAdd
// add
impl<N: Scalar + RealField> std::ops::Add for Euler<N> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Euler::new(self.p + other.p, self.y + other.y, self.r + other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Add<N> for Euler<N> {
    type Output = Self;

    #[inline]
    fn add(self, other: N) -> Self {
        Euler::new(self.p + other, self.y + other, self.r + other)
    }
}

impl<N: Scalar + RealField> std::ops::AddAssign for Euler<N> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p + other.p,
            y: self.y + other.y,
            r: self.r + other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::AddAssign<N> for Euler<N> {
    #[inline]
    fn add_assign(&mut self, other: N) {
        *self = Self {
            p: self.p + other,
            y: self.y + other,
            r: self.r + other,
        };
    }
}

// mul
impl<N: Scalar + RealField> std::ops::Mul for Euler<N> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Euler::new(self.p * other.p, self.y * other.y, self.r * other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Mul<N> for Euler<N> {
    type Output = Self;

    #[inline]
    fn mul(self, other: N) -> Self {
        Euler::new(self.p * other, self.y * other, self.r * other)
    }
}

impl<N: Scalar + RealField> std::ops::MulAssign for Euler<N> {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p * other.p,
            y: self.y * other.y,
            r: self.r * other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::MulAssign<N> for Euler<N> {
    #[inline]
    fn mul_assign(&mut self, other: N) {
        *self = Self {
            p: self.p * other,
            y: self.y * other,
            r: self.r * other,
        };
    }
}

// sub
impl<N: Scalar + RealField> std::ops::Sub for Euler<N> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Euler::new(self.p - other.p, self.y - other.y, self.r - other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Sub<N> for Euler<N> {
    type Output = Self;

    #[inline]
    fn sub(self, other: N) -> Self {
        Euler::new(self.p - other, self.y - other, self.r - other)
    }
}

impl<N: Scalar + RealField> std::ops::SubAssign for Euler<N> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p - other.p,
            y: self.y - other.y,
            r: self.r - other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::SubAssign<N> for Euler<N> {
    #[inline]
    fn sub_assign(&mut self, other: N) {
        *self = Self {
            p: self.p - other,
            y: self.y - other,
            r: self.r - other,
        };
    }
}

// div
impl<N: Scalar + RealField> std::ops::Div for Euler<N> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        Euler::new(self.p / other.p, self.y / other.y, self.r / other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Div<N> for Euler<N> {
    type Output = Self;

    #[inline]
    fn div(self, other: N) -> Self {
        Euler::new(self.p / other, self.y / other, self.r / other)
    }
}

impl<N: Scalar + RealField> std::ops::DivAssign for Euler<N> {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p / other.p,
            y: self.y / other.y,
            r: self.r / other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::DivAssign<N> for Euler<N> {
    #[inline]
    fn div_assign(&mut self, other: N) {
        *self = Self {
            p: self.p / other,
            y: self.y / other,
            r: self.r / other,
        };
    }
}
