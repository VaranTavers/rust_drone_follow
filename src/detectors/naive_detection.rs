use opencv as cv;
use cv::core::*;
use cv::prelude::*;
use opencv::imgproc::{contour_area};

use crate::opencv_custom::{MyColor, get_contour};
use crate::traits::Detector;

pub struct NaiveDetection {
    lower_color: MyColor,
    upper_color: MyColor,
    lower_size: f64,
    point: Option<Point>,
    cert: f64,
}

impl NaiveDetection {
    pub fn new((lower_color, upper_color): (MyColor, MyColor), lower_size: f64) -> NaiveDetection {
        NaiveDetection {
            lower_color,
            upper_color,
            lower_size,
            point: None,
            cert: 0.0
        }
    }
}

impl Detector for NaiveDetection {
    fn estimate_new_position(&mut self, img: &Mat, _old_pos: Option<&Point>) {

        let contours = get_contour(img, &self.lower_color, &self.upper_color);

        let (point, _cert) = get_point_from_contours(&contours, self.lower_size);

        self.point = point;
        self.cert = 1.0;
    }

    fn get_estimated_position(&self) -> Option<Point> {
        self.point.map(|a| a.clone())
    }

    fn get_estimated_certainty(&self) -> f64 {
        self.cert
    }

    fn draw_on_image(&self, _img: &mut Mat) {
        
    }
}

fn get_point_from_contours(contours: &cv::types::VectorOfVectorOfPoint, lower_size: f64) -> (Option<Point>, u32) {
    let c_with_area = contours.iter()
        .map(|contour| (contour_area(&contour, false).unwrap(), contour))
        .collect::<Vec<(f64, cv::types::VectorOfPoint)>>();

    if contours.len() > 0 {
        let (biggest_area, biggest) = c_with_area.iter()
            .fold((-1.0, None), |(acc_a, acc_c), (c_a, c_c)| {
                if *c_a > acc_a {
                    return (*c_a, Some(c_c));
                }
                (acc_a, acc_c)
            });

        if biggest_area > lower_size {
            match biggest {
                Some(contour) => {
                    let (s_x, s_y) = contour.iter()
                        .fold((0, 0), |(ax, ay), c_p| (ax + c_p.x * 2, ay + c_p.y * 2));
                    let l = contour.len() as i32;

                    return (Some(Point::new(s_x / l, s_y / l)), 1);
                }
                _ => {

                }
            }
        }
    }

    (None, 0)
}
