use crate::opencv_custom::MyColor;

pub struct Hat {
    pub color_low  : MyColor,
    pub color_high : MyColor,
    pub size_avg   : f64,
}

impl Hat {
    pub fn new(color_low: MyColor, color_high: MyColor, size_avg: f64) -> Hat {
        Hat {
            color_low,
            color_high,
            size_avg
        }
    }
}