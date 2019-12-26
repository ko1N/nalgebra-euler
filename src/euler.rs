use super::float::NormalizeAngle;
use super::vector3::VectorAngles;
use super::IsFinite;
use alga::general::RealField;
use nalgebra::{Scalar, Vector3};

pub trait AngleVectors<N: Scalar> {
    fn forward(&self) -> Vector3<N>;
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
        //let r = self.r.to_radians().sin_cos();

        // TODO: right / up ?

        Vector3::new(p.1 * y.1, -p.0, p.1 * y.0).normalize()
    }
}

impl AngleVectors<f64> for Euler<f64> {
    fn forward(&self) -> Vector3<f64> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        //let r = self.r.to_radians().sin_cos();

        // TODO: right / up ?

        Vector3::new(p.1 * y.1, -p.0, p.1 * y.0).normalize()
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
