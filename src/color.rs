use crate::vector::Vector3d;

#[derive(Debug)]
pub struct RgbReal {
    pub rgb: Vector3d,
}

impl RgbReal {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            rgb: Vector3d::new(red, green, blue),
        }
    }

    pub fn red(&self) -> f64 {
        self.rgb.x
    }

    pub fn green(&self) -> f64 {
        self.rgb.y
    }

    pub fn blue(&self) -> f64 {
        self.rgb.z
    }
}

pub struct RgbInt {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl From<RgbReal> for RgbInt {
    /// scale from [0.0, 1.0] to Z ∩ [0, 255].
    /// There is no way to completely evenly do this (there will always be at least a
    /// one point discrepancy), but this should be very close
    fn from(color: RgbReal) -> Self {
        // I want to scale a number in the range [0.0, 1.0] to [0, 255] (having 1 map to 255).
        // the problem is that I want to convert safely (using try_from)
        let scaled_red: u16 = (color.red() * 255f64).clamp(0f64, 255f64).floor() as u16;
        let scaled_green: u16 = (color.green() * 255f64).clamp(0f64, 255f64).floor() as u16;
        let scaled_blue: u16 = (color.blue() * 255f64).clamp(0f64, 255f64).floor() as u16;
        Self {
            red: scaled_red,
            green: scaled_green,
            blue: scaled_blue,
        }
    }
}

impl From<RgbInt> for RgbReal {
    /// Scaled from the integers in [0, 255] to a real number [0, 1.0] inclusive.
    fn from(color: RgbInt) -> Self {
        let scaled_red = f64::from(color.red) / 255f64;
        let scaled_green = f64::from(color.green) / 255f64;
        let scaled_blue = f64::from(color.blue) / 255f64;
        Self::new(scaled_red, scaled_green, scaled_blue)
    }
}

impl RgbInt {
    pub fn to_ppm_pixel(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}
