pub trait NormalizeAngle {
    fn normalize_angle(self) -> Self;
}

impl NormalizeAngle for f32 {
    fn normalize_angle(self) -> Self {
        if !self.is_finite() {
            0f32
        } else if self >= -180f32 && self <= 180f32 {
            self
        } else {
            let rots = (self / 360f32).abs().round();
            if self < 0f32 {
                self + (360f32 * rots)
            } else {
                self - (360f32 * rots)
            }
        }
    }
}

impl NormalizeAngle for f64 {
    fn normalize_angle(self) -> Self {
        if !self.is_finite() {
            0f64
        } else if self >= -180f64 && self <= 180f64 {
            self
        } else {
            let rots = (self / 360f64).abs().round();
            if self < 0f64 {
                self + (360f64 * rots)
            } else {
                self - (360f64 * rots)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_normalize_angle() {
        assert_approx_eq!((480f32).normalize_angle(), 120f32);
        assert_approx_eq!((-480f64).normalize_angle(), -120f64);
        assert_approx_eq!((-960f32).normalize_angle(), 120f32);
        assert_approx_eq!((360f32).normalize_angle(), 0f32);
        assert_approx_eq!((-360f64).normalize_angle(), 0f64);
        assert_approx_eq!((180f32).normalize_angle(), 180f32);
        assert_approx_eq!((-180f64).normalize_angle(), -180f64);
    }
}
