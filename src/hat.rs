use crate::opencv_custom::LabColor;

pub struct Hat {
    pub color_low  : LabColor,
    pub color_high : LabColor,
    pub size_avg   : f64,
}

impl Hat {
    pub fn new(color_low: LabColor, color_high: LabColor, size_avg: f64) -> Hat {
        Hat {
            color_low,
            color_high,
            size_avg
        }
    }
}