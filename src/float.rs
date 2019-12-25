pub trait NormalizeAngle {
    fn normalize_angle(self) -> Self;
}

impl NormalizeAngle for f32 {
    fn normalize_angle(self) -> Self {
        if !self.is_finite() {
            0.0
        } else if self >= -180.0 && self <= 180.0 {
            self
        } else {
            let rots = (self / 360.0).abs().round();
            if self < 0.0 {
                self + (360.0 * rots)
            } else {
                self - (360.0 * rots)
            }
        }
    }
}
