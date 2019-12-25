use super::float::SinCos;
use nalgebra::{Scalar, Vector3};

// TODO: Euler or Angle Type

/*
    inline bool
    is_valid(void) const {
        return (std::isfinite(this->x) && std::isfinite(this->y) && std::isfinite(this->z));
    }

    FORCEINLINE float
    length(void) const {
        return math::sqrtf((this->x * this->x) + (this->y * this->y) + (this->z * this->z));
    }

    FORCEINLINE float
    length_sqr(void) const {
        return (this->x * this->x) + (this->y * this->y) + (this->z * this->z);
    }

    FORCEINLINE float
    length_2d(void) const {
        return math::sqrtf((this->x * this->x) + (this->y * this->y));
    }

    FORCEINLINE float
    length_2d_sqr(void) const {
        return (this->x * this->x) + (this->y * this->y);
    }

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
/*
euler
euler::lerp(const euler &other, float amount) const {
    vector3 v1 = this->to_vectors();
    vector3 v2 = other.to_vectors();

    vector3 delta = v2 - v1;
    float len = delta.length();
    delta /= amount;

    if (delta.length() < std::numeric_limits<float>::epsilon() || delta.length() > len) {
        return *this;
    }

    vector3 target = v1 + delta;
    euler out = target.to_euler().sanitized_in_place();
    if (!out.is_valid()) {
        return *this;
    }

    return out;
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

pub trait AngleVectors<N: Scalar> {
    fn forward(&self) -> Vector3<N>;
}

impl AngleVectors<f32> for (f32, f32, f32) {
    fn forward(&self) -> Vector3<f32> {
        let p = self.0.to_radians().sin_cos();
        let y = self.1.to_radians().sin_cos();
        let r = self.2.to_radians().sin_cos();

        // right / up ?
/*
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
*/

        // return {cp * cy, cp * sy, -sp};
        Vector3::new(p.1 * y.1, -p.0, p.1 * y.0)
    }
}
