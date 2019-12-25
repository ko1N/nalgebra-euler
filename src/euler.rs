use super::vector3::VectorAngles;
use alga::general::RealField;
use nalgebra::{Scalar, Vector3};

// TODO: Euler or Angle Type
// pub type Vector3<N> = VectorN<N, U3>;

pub trait AngleVectors<N: Scalar> {
    fn forward(&self) -> Vector3<N>;
}

pub trait IsFinite {
    fn is_finite(&self) -> bool;
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

// TODO: add IsFinite trait for Euler and Vector3
impl<N: Scalar + RealField> IsFinite for Euler<N> {
    fn is_finite(&self) -> bool {
        self.p.is_finite() && self.y.is_finite() && self.r.is_finite()
    }
}

// TODO: create to_radians and sin_cos impl on N ?
impl AngleVectors<f32> for Euler<f32> {
    fn forward(&self) -> Vector3<f32> {
        let p = self.p.to_radians().sin_cos();
        let y = self.y.to_radians().sin_cos();
        //let r = self.r.to_radians().sin_cos();

        // TODO: right / up ?

        Vector3::new(p.1 * y.1, -p.0, p.1 * y.0)
    }
}

impl Euler<f32> {
    pub fn lerp(&self, other: &Euler<f32>, amount: f32) -> Euler<f32> {
        let v1 = self.forward();
        let v2 = other.forward();

        let mut delta = v2 - v1;
        let mag = delta.magnitude();
        delta /= amount;
        let new_mag = delta.magnitude();

        if new_mag < std::f32::EPSILON || new_mag > mag {
            self.clone()
        } else {
            let t = v1 + delta;
            let out = t.euler_angles(); /* TODO: normalize */
            if !out.is_finite() {
                self.clone()
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

/*
vector3
euler::to_vectors(vector3 *right, vector3 *up) const {
    float sr, sp, sy, cr, cp, cy;

    math::sincos(math::deg_to_rad(this->y), sy, cy);
    math::sincos(math::deg_to_rad(this->x), sp, cp);
    math::sincos(math::deg_to_rad(this->z), sr, cr);

    if (right) {
        right->x = -1.0f * sr * sp * cy + -1.0f * cr * -sy;
        right->y = -1.0f * sr * sp * sy + -1.0f * cr * cy;
        right->z = -1.0f * sr * cp;
    }

    if (up) {
        up->x = cr * sp * cy + -sr * -sy;
        up->y = cr * sp * sy + -sr * cy;
        up->z = cr * cp;
    }

    // forward vector
    return {cp * cy, cp * sy, -sp};
}
*/
