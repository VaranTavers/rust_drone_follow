use crate::models::lab_color::LabColor;

/// This struct contains the necessary information for a NaiveDetector about the hat that the
/// person, that should be followed, wears. It requires two Color coordinates from the Lab color space
/// and the average size of the hat.
pub struct Hat {
    pub color_low  : LabColor,
    pub color_high : LabColor,
    pub size_avg   : f64,
}

impl Hat {
    /// Creates a new Hat struct
    ///
    /// Usage:
    /// ```
    /// use rust_drone_follow::models::hat::Hat;
    /// use rust_drone_follow::models::lab_color::LabColor;
    /// // ...
    /// # fn main() {
    ///    let hat = Hat::new(
    ///         LabColor::new(0, 20, -127),
    ///         LabColor::new(80, 127, -20),
    ///         1200.0
    ///     );
    /// # }
    ///
    /// ```
    pub fn new(color_low: LabColor, color_high: LabColor, size_avg: f64) -> Hat {
        Hat {
            color_low,
            color_high,
            size_avg
        }
    }
}