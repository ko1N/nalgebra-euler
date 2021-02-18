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

// TODO: implement from and to traits
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Euler<N: Scalar> {
    pub p: N,
    pub y: N,
    pub r: N,
}

impl<N: Scalar> Euler<N> {
    pub fn new(p: N, y: N, r: N) -> Self {
        Self { p, y, r }
    }
}

impl<N: Scalar + RealField> IsFinite for Euler<N> {
    fn is_finite(&self) -> bool {
        self.p.is_finite() && self.y.is_finite() && self.r.is_finite()
    }
}

impl Euler<f32> {
    pub fn normalize(&self) -> Self {
        Euler::new(
            self.p.normalize_angle(),
            self.y.normalize_angle(),
            self.r.normalize_angle(),
        )
    }
}

impl Euler<f64> {
    pub fn normalize(&self) -> Self {
        Euler::new(
            self.p.normalize_angle(),
            self.y.normalize_angle(),
            self.r.normalize_angle(),
        )
    }
}

// TODO: create to_radians and sin_cos impl on N ?
impl AngleVectors<f32> for Euler<f32> {
    fn forward(&self) -> Vector3<f32> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();

        Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
            .swap_yz()
            .normalize()
    }

    fn right(&self) -> Vector3<f32> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        let r = self.r.to_radians().sin_cos();

        Vector3::new(
            -1f32 * r.0 * p.0 * y.1 + -1f32 * r.1 * -y.0,
            -1f32 * r.0 * p.0 * y.0 + -1f32 * r.1 * y.1,
            -1f32 * r.0 * p.1,
        )
        .swap_yz()
        .normalize()
    }

    fn up(&self) -> Vector3<f32> {
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

    fn vectors(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        let r = self.r.to_radians().sin_cos();

        let forward = Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
            .swap_yz()
            .normalize();

        let right = Vector3::new(
            -1f32 * r.0 * p.0 * y.1 + -1f32 * r.1 * -y.0,
            -1f32 * r.0 * p.0 * y.0 + -1f32 * r.1 * y.1,
            -1f32 * r.0 * p.1,
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

impl AngleVectors<f64> for Euler<f64> {
    fn forward(&self) -> Vector3<f64> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();

        Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
            .swap_yz()
            .normalize()
    }

    fn right(&self) -> Vector3<f64> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        let r = self.r.to_radians().sin_cos();

        Vector3::new(
            -1f64 * r.0 * p.0 * y.1 + -1f64 * r.1 * -y.0,
            -1f64 * r.0 * p.0 * y.0 + -1f64 * r.1 * y.1,
            -1f64 * r.0 * p.1,
        )
        .swap_yz()
        .normalize()
    }

    fn up(&self) -> Vector3<f64> {
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

    fn vectors(&self) -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        let r = self.r.to_radians().sin_cos();

        let forward = Vector3::new(p.1 * y.1, p.1 * y.0, -p.0)
            .swap_yz()
            .normalize();

        let right = Vector3::new(
            -1f64 * r.0 * p.0 * y.1 + -1f64 * r.1 * -y.0,
            -1f64 * r.0 * p.0 * y.0 + -1f64 * r.1 * y.1,
            -1f64 * r.0 * p.1,
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

impl Euler<f32> {
    pub fn lerp(&self, other: &Euler<f32>, amount: f32) -> Euler<f32> {
        let v1 = self.forward().normalize();
        let v2 = other.forward().normalize();

        let mut delta = v2 - v1;
        let mag = delta.magnitude();
        delta /= amount;
        let new_mag = delta.magnitude();

        if new_mag < std::f32::EPSILON || new_mag > mag {
            // prevent overshoot
            other.clone()
        } else {
            let t = v1 + delta;
            let out = t.euler_angles().normalize();
            if !out.is_finite() {
                other.clone()
            } else {
                out
            }
        }
    }
}

impl Euler<f64> {
    pub fn lerp(&self, other: &Euler<f64>, amount: f64) -> Euler<f64> {
        let v1 = self.forward().normalize();
        let v2 = other.forward().normalize();

        let mut delta = v2 - v1;
        let mag = delta.magnitude();
        delta /= amount;
        let new_mag = delta.magnitude();

        if new_mag < std::f64::EPSILON || new_mag > mag {
            // prevent overshoot
            other.clone()
        } else {
            let t = v1 + delta;
            let out = t.euler_angles().normalize();
            if !out.is_finite() {
                other.clone()
            } else {
                out
            }
        }
    }
}

// TODO: use ClosedAdd
// add
impl<N: Scalar + RealField> std::ops::Add for Euler<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Euler::new(self.p + other.p, self.y + other.y, self.r + other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Add<N> for Euler<N> {
    type Output = Self;

    fn add(self, other: N) -> Self {
        Euler::new(self.p + other, self.y + other, self.r + other)
    }
}

impl<N: Scalar + RealField> std::ops::AddAssign for Euler<N> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p + other.p,
            y: self.y + other.y,
            r: self.r + other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::AddAssign<N> for Euler<N> {
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

    fn mul(self, other: Self) -> Self {
        Euler::new(self.p * other.p, self.y * other.y, self.r * other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Mul<N> for Euler<N> {
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Euler::new(self.p * other, self.y * other, self.r * other)
    }
}

impl<N: Scalar + RealField> std::ops::MulAssign for Euler<N> {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p * other.p,
            y: self.y * other.y,
            r: self.r * other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::MulAssign<N> for Euler<N> {
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

    fn sub(self, other: Self) -> Self {
        Euler::new(self.p - other.p, self.y - other.y, self.r - other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Sub<N> for Euler<N> {
    type Output = Self;

    fn sub(self, other: N) -> Self {
        Euler::new(self.p - other, self.y - other, self.r - other)
    }
}

impl<N: Scalar + RealField> std::ops::SubAssign for Euler<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p - other.p,
            y: self.y - other.y,
            r: self.r - other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::SubAssign<N> for Euler<N> {
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

    fn div(self, other: Self) -> Self {
        Euler::new(self.p / other.p, self.y / other.y, self.r / other.r)
    }
}

impl<N: Scalar + RealField> std::ops::Div<N> for Euler<N> {
    type Output = Self;

    fn div(self, other: N) -> Self {
        Euler::new(self.p / other, self.y / other, self.r / other)
    }
}

impl<N: Scalar + RealField> std::ops::DivAssign for Euler<N> {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            p: self.p / other.p,
            y: self.y / other.y,
            r: self.r / other.r,
        };
    }
}

impl<N: Scalar + RealField> std::ops::DivAssign<N> for Euler<N> {
    fn div_assign(&mut self, other: N) {
        *self = Self {
            p: self.p / other,
            y: self.y / other,
            r: self.r / other,
        };
    }
}

/*
    inline void
    normalize(void) {
        for (int i = 0; i < 3; i++) {
            math::normalize_angle(this->base[i]);
        }
    }

    inline euler
    normalized(void) const {
        euler temp = *this;
        temp.normalize();
        return temp;
    }

    inline euler &
    normalized_in_place(void) {
        this->normalize();
        return *this;
    }

    inline void
    sanitize(void) {
        this->normalize();
        math::clamp<float>(this->pitch, -90.0f, 90.0f);
        math::normalize_angle(this->yaw);
        math::clamp<float>(this->roll, -90.0f, 90.0f);
    }

    inline euler
    sanitized(void) const {
        euler temp = *this;
        temp.sanitize();
        return temp;
    }

    inline euler &
    sanitized_in_place(void) {
        this->sanitize();
        return *this;
    }

    euler lerp(const euler &ang, float amount) const;

    static euler
    lerp(const euler &first, const euler &second, float amount) {
        return first.lerp(second, amount);
    }
*/

// https://github.com/divideconcept/FastTrigo
