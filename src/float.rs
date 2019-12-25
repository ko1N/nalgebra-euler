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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_angle() {
        assert_eq!((480.0).normalize_angle(), 120.0);
        assert_eq!((-480.0).normalize_angle(), -120.0);
        assert_eq!((-960.0).normalize_angle(), 120.0);
        assert_eq!((360.0).normalize_angle(), 0.0);
        assert_eq!((-360.0).normalize_angle(), 0.0);
        assert_eq!((180.0).normalize_angle(), 180.0);
        assert_eq!((-180.0).normalize_angle(), -180.0);
    }
}
