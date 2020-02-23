use opencv as cv;
use cv::core::*;
use cv::prelude::*;
use opencv::imgproc::{COLOR_BGR2GRAY, cvt_color, threshold, THRESH_BINARY, THRESH_OTSU, canny, hough_lines_p, gaussian_blur};
use opencv::types::{VectorOfVec4f};

const PI: f64 = std::f64::consts::PI;

pub fn mat_size_of_other(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), mat.typ().unwrap())
        .unwrap().to_mat().unwrap()
}

pub fn mat_size_of_other_cv_8u(mat: &Mat) -> Mat {
    Mat::zeros_size(mat.size().unwrap(), CV_8U)
        .unwrap().to_mat().unwrap()
}

pub fn grayscale(image: &Mat) -> Mat {
    let mut grey = mat_size_of_other(image);
    cvt_color(image, &mut grey, COLOR_BGR2GRAY, 0).unwrap();
    grey
}

pub fn gaussian_blur_phase(image: &Mat) -> Mat {
    let mut blurred = mat_size_of_other(image);
    gaussian_blur(image, &mut blurred, Size::new(5, 5),
                  0.0, 0.0, BORDER_DEFAULT).unwrap();
    blurred
}

pub fn canny_interval(image: &Mat) -> (f64, f64) {
    let mut thresh = mat_size_of_other(image);
    let high_thresh = threshold(&image, &mut thresh, 0.0, 255.0,
                                THRESH_BINARY + THRESH_OTSU).unwrap();
    (high_thresh / 2.0, high_thresh)
}

pub fn canny_phase(image: &Mat) -> Mat {
    let mut cannied = mat_size_of_other(image);
    let (low_thresh, high_thresh) = canny_interval(&image);
    canny(&image, &mut cannied, low_thresh, high_thresh, 3, false).unwrap();

    cannied
}

pub fn hough_phase(image: &Mat) -> VectorOfVec4f {
    let mut lines: VectorOfVec4f = VectorOfVec4f::new();
    hough_lines_p(&image, &mut lines, 1.0, PI / 180.0, 100, 0.0, 5.0).unwrap();

    lines
}
