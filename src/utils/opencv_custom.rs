use std::clone::Clone;

use opencv::core::*;
use opencv::imgproc::{COLOR_BGR2Lab, cvt_color, LINE_8, line};

use crate::models::lab_color::LabColor;

/// Creates a new Matrix with the same size and same type as the original.
pub fn mat_size_of_other(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), mat.typ().unwrap())
        .unwrap().to_mat().unwrap()
}

/// Creates a new Matrix with the same size as the original but with CV_8U as type.
pub fn mat_size_of_other_cv_8u(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), CV_8U)
        .unwrap().to_mat().unwrap()
}

/// Returns a mask (result of in_range) of the image, with everything that is between the two given colors.
pub fn get_mask(img: &Mat, lower_c: &LabColor, upper_c: &LabColor) -> Mat {
    let lower = Mat::from_slice::<u8>(&[lower_c.l, lower_c.a, lower_c.b]).unwrap();
    let upper = Mat::from_slice::<u8>(&[upper_c.l, upper_c.a, upper_c.b]).unwrap();

    let mut mask: Mat = mat_size_of_other_cv_8u(&img);

    opencv::core::in_range(&img, &lower, &upper, &mut mask).unwrap();

    mask
}

/// Returns a vector of contours (VectorOfPoint) of objects from the picture that are between the
/// given colors.
pub fn get_contours(a: &Mat, lower_bound: &LabColor, upper_bound: &LabColor) -> opencv::types::VectorOfVectorOfPoint {
    let mut hsv = mat_size_of_other(a);
    cvt_color(a, &mut hsv, COLOR_BGR2Lab, 0).unwrap();

    let mask = get_mask(&hsv, lower_bound, upper_bound);

    let mut output: Mat = mat_size_of_other(&hsv);
    let mut thresh: Mat = mat_size_of_other(&hsv);

    opencv::core::bitwise_and(&a, &a, &mut output, &mask).unwrap();
    opencv::imgproc::threshold(&mask, &mut thresh, 40.0, 255.0, 0).unwrap();

    let mut contours: opencv::types::VectorOfVectorOfPoint = opencv::prelude::Vector::new();

    opencv::imgproc::find_contours(&thresh, &mut contours,
                               opencv::imgproc::RETR_EXTERNAL,
                               opencv::imgproc::CHAIN_APPROX_NONE,
                               Point::new(0, 0)).unwrap();

    contours
}

/// A simplified function to call line which deals with cloning the points and unwrapping the result
pub fn line_c (img: &mut Mat, a: &Point, b: &Point, color: Scalar) {
    line(img, a.clone(), b.clone(), color, 2, LINE_8, 0).unwrap();
}

/// A function that always returns a scalar containing the color red (255, 0, 0)
pub fn get_red()-> Scalar {
    Scalar::new(0.0, 0.0, 255.0, 255.0)
}

/// A function that always returns a scalar containing the color blue (0, 0, 255)
pub fn get_blue()-> Scalar {
    Scalar::new(255.0, 0.0, 0.0, 255.0)
}

/// A function that always returns a scalar containing the color green (0, 255, 0)
pub fn get_green()-> Scalar {
    Scalar::new(0.0, 255.0, 0.0, 255.0)
}
