/// A struct that holds information for a color in Lab colorspace.
pub struct LabColor {
    pub l: u8,
    pub a: u8,
    pub b: u8,
}

impl LabColor {
    /// Creates a new LabColor.
    ///
    /// l: Ranges from 0 to 100 (dark -> light)
    ///
    /// a: Ranges from -127 to 127 (green -> red)
    ///
    /// b: Ranges from -127 to 127 (blue -> yellow)
    ///
    /// These values will be converted to values OpenCV understands.
    pub fn new(l: i8, a: i8, b: i8) -> LabColor {
        let ll;
        if l > 100 || l < 0 {
            ll = 100;
        } else {
            ll = l as i32;
        }
        LabColor {
            l: (ll * 255 / 100) as u8,
            a: (a as i32 + 128) as u8,
            b: (b as i32 + 128) as u8,
        }
    }
}
