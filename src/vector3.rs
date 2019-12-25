use super::float::NormalizeAngle;
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
    fn forward_to_euler(&self) -> (N, N, N);
}

impl VectorAngles<f32> for Vector3<f32> {
    fn forward_to_euler(&self) -> (f32, f32, f32) {
        if self.x == 0.0 && self.z == 0.0 {
            let pitch = {
                if self.y > 0.0 {
                    270.0
                } else {
                    90.0
                }
            };
            let yaw = -90.0;
            ((-pitch).normalize_angle(), yaw, 0.0)
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

            (
                (-pitch).normalize_angle(),
                (yaw - 90.0).normalize_angle(),
                0.0,
            )
        }
    }
}

/*
euler
vector3::to_euler(void) const {
    float pitch, yaw;

    if (this->x == 0.0f && this->y == 0.0f) {
        pitch = this->z > 0.0f ? 270.0f : 90.0f;
        yaw = 0.0f;
    } else {
        pitch = math::rad_to_deg(std::atan2(-this->z, this->length_2d()));
        if (pitch < 0.0f)
            pitch += 360.0f;

        yaw = math::rad_to_deg(std::atan2(this->y, this->x));
        if (yaw < 0.0f)
            yaw += 360.0f;
    }

    return {pitch, yaw, 0.0f};
}

euler
vector3::to_euler(const vector3 &pseudo_up) const {
    float pitch, yaw, roll;

    vector3 left = pseudo_up.cross_product(*this);
    left.normalize();

    const float dist2d = this->length_2d();

    pitch = math::rad_to_deg(std::atan2(-this->z, dist2d));

    if (dist2d > 0.001f) {
        const float up_z = (this->x * left.y) - (this->y * left.x);

        yaw = math::rad_to_deg(std::atan2(this->y, this->x));
        roll = math::rad_to_deg(std::atan2(left.z, up_z));
    } else {
        yaw = math::rad_to_deg(std::atan2(-left.x, left.y));
        roll = 0.0f;
    }

    return {pitch, yaw, roll};
}
*/
