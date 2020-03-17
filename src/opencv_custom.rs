use opencv as cv;
use cv::core::*;
use opencv::imgproc::{COLOR_BGR2Lab, cvt_color, LINE_8, line};
use std::clone::Clone;

/// A new type for points that ensures type-safety: No point in the image's coordinate system gets into
/// calculations and no point from calculations gets used as an input to an OpenCV drawing function.
pub struct GeometricPoint {
    pub x: i32,
    pub y: i32
}

impl Clone for GeometricPoint {
    fn clone(&self) -> Self {
        GeometricPoint {
            x: self.x,
            y: self.y,
        }
    }
}

impl GeometricPoint {
    pub fn new(x: i32, y: i32) -> GeometricPoint {
        GeometricPoint { x, y }
    }
    pub fn d(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

pub fn mat_size_of_other(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), mat.typ().unwrap())
        .unwrap().to_mat().unwrap()
}

pub fn mat_size_of_other_cv_8u(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), CV_8U)
        .unwrap().to_mat().unwrap()
}

pub struct MyColor {
    pub l: u8,
    pub a: u8,
    pub b: u8,
}

impl MyColor {
    /// l: Ranges from 0 to 100
    /// a: Ranges from -127 to 127
    /// b: Ranges from -127 to 127
    pub fn new(l: i8, a: i8, b: i8) -> MyColor {
        let ll;
        if l > 100 || l < 0 {
            ll = 100;
        } else {
            ll = l as i32;
        }
        MyColor {
            l: (ll * 255 / 100) as u8,
            a: (a as i32 + 128) as u8, 
            b: (b as i32 + 128) as u8, 
        }
    }
}

pub fn get_mask(img: &Mat, lower_c: &MyColor, upper_c: &MyColor) -> Mat {
    let lower = Mat::from_slice::<u8>(&[lower_c.l, lower_c.a, lower_c.b]).unwrap();
    let upper = Mat::from_slice::<u8>(&[upper_c.l, upper_c.a, upper_c.b]).unwrap();

    let mut mask: Mat = mat_size_of_other_cv_8u(&img);

    cv::core::in_range(&img, &lower, &upper, &mut mask).unwrap();

    mask
}

pub fn get_contours(a: &Mat, lower_bound: &MyColor, upper_bound: &MyColor) -> cv::types::VectorOfVectorOfPoint {
    let mut hsv = mat_size_of_other(a);
    cvt_color(a, &mut hsv, COLOR_BGR2Lab, 0).unwrap();

    let mask = get_mask(&hsv, lower_bound, upper_bound);
    let mut output: Mat = mat_size_of_other(&hsv);

    let mut thresh: Mat = mat_size_of_other(&hsv);

    cv::core::bitwise_and(&a, &a, &mut output, &mask).unwrap();
    cv::imgproc::threshold(&mask, &mut thresh, 40.0, 255.0, 0).unwrap();

    let mut contours: cv::types::VectorOfVectorOfPoint = cv::prelude::Vector::new();

    cv::imgproc::find_contours(&thresh, &mut contours,
                               cv::imgproc::RETR_EXTERNAL,
                               cv::imgproc::CHAIN_APPROX_NONE,
                               Point::new(0, 0)).unwrap();

    contours
}

pub fn line_c (img: &mut Mat, a: &Point, b: &Point, color: Scalar) {
    line(img, a.clone(), b.clone(), color, 2, LINE_8, 0).unwrap();
}

pub fn get_red()-> Scalar {
    Scalar::new(0.0, 0.0, 255.0, 255.0)
}

pub fn get_blue()-> Scalar {
    Scalar::new(255.0, 0.0, 0.0, 255.0)
}

pub fn get_green()-> Scalar {
    Scalar::new(0.0, 255.0, 0.0, 255.0)
}
