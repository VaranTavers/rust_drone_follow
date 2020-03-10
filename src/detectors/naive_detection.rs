use opencv as cv;
use cv::core::*;
use cv::prelude::*;
use cv::highgui::*;
use opencv::imgproc::{contour_area, line, LINE_8, circle};

use crate::opencv_custom::{MyColor, get_contours, mat_size_of_other};
use crate::traits::Detector;
use opencv::types::VectorOfVectorOfPoint;
use opencv::types::VectorOfPoint;

const PI: f64 = std::f64::consts::PI;

enum TanableAngle {
    Vertical,
    Angle(f64)
}

pub struct NaiveDetection {
    lower_color: MyColor,
    upper_color: MyColor,
    lower_size: f64,
    point: Option<Point>,
    cert: f64,
    angle: TanableAngle,
    tmp_points: (i32, i32, i32, i32)
}

impl NaiveDetection {
    pub fn new((lower_color, upper_color): (MyColor, MyColor), lower_size: f64) -> NaiveDetection {
        NaiveDetection {
            lower_color,
            upper_color,
            lower_size,
            point: None,
            cert: 0.0,
            angle: TanableAngle::Angle(0.0),
            tmp_points: (0, 0, 0, 0)
        }
    }
}

impl Detector for NaiveDetection {
    fn get_detected_position(&self) -> Option<Point> {
        self.point.map(|a| a.clone())
    }

    fn get_detected_angle(&self) -> Option<f64> {
        match self.angle {
            TanableAngle::Angle(i) => {
                Some(i)
            }
            TanableAngle::Vertical => {
                Some(PI / 2.0)
            }
        }
    }

    fn get_detection_certainty(&self) -> f64 {
        self.cert
    }

    fn detect_new_position(&mut self, img: &Mat, _old_pos: Option<Point>) {

        let contours = get_contours(img, &self.lower_color, &self.upper_color);

        let (point, cert, angle) = get_point_from_contours(&contours, self.lower_size);

        self.point = point;
        self.cert = cert as f64;
        self.angle = angle;
    }

    fn draw_on_image(&self, img: &mut Mat) {
        let k = 15;
        match self.point {
            Some(p) => {
                let c_point = p.clone();
                let other_point;
                match self.angle {
                    TanableAngle::Angle(angle) => {
                        let tan_angle = angle.tan() as i32;
                        other_point = Point::new(c_point.x + k, c_point.y + k * tan_angle);
                    }
                    TanableAngle::Vertical => {
                        other_point = Point::new(c_point.x, c_point.y - k);
                    }
                }

                line(img,
                     c_point,
                     other_point,
                     Scalar::new(0.0, 0.0, 255.0, 255.0),
                     2, LINE_8, 1).unwrap();
            }
            None => {
            }
        }
    }
}

fn get_point_from_contours(contours: &cv::types::VectorOfVectorOfPoint, lower_size: f64) -> (Option<Point>, u32, TanableAngle) {
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
                    let (c_x, c_y) = (s_x / l, s_y / l);

                    return (Some(Point::new(c_x, c_y)), 1, get_angle((c_x, c_y), contour));
                }
                _ => {

                }
            }
        }
    }

    (None, 0, TanableAngle::Angle(0.0))
}

fn get_angle((c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> TanableAngle {
    let ((a_x, a_y), (b_x, b_y)) = get_points_from_two_sides((c_x, c_y), contour);

    if a_x == b_x {
        return TanableAngle::Vertical;
    }
    TanableAngle::Angle((a_y - b_y) as f64 / (a_x - b_x) as f64)
}

/// Gets the two closest points to the center of the hat, which are at least a constant far away from
/// eachother.
/// This is done by finding the closest point (A) and then finding the second point (B) so that
/// the center point (C) is the closest to the AB line
fn get_points_from_two_sides((c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> ((i32, i32), (i32, i32)) {
    let (cl_x, cl_y, _) = get_closest_point_to_center_from_contour((c_x, c_y), contour);

    let (other_x, other_y, _) = contour.iter()
        .fold((0, 0, 500000.0), |(dx, dy, d), c_p| {
            let dist = get_distance_from_line((cl_x, cl_y), (c_p.x, c_p.y), (c_x, c_y));
            if dist < d {
                return (c_p.x, c_p.y, dist);
            }
            (dx, dy, d)
        });

    ((cl_x, cl_y), (other_x, other_y))
}

fn get_closest_point_to_center_from_contour((c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> (i32, i32, f64) {
    contour.iter()
        .fold((0, 0, 500000.0), |(dx, dy, d), c_p| {
            let dist_x = c_x - c_p.x;
            let dist_y = c_y - c_p.y;
            let dist = ((dist_x.pow(2) + dist_y.pow(2)) as f64).sqrt();
            if dist < d {
                return (c_p.x, c_p.y, dist);
            }
            return (dx, dy, d);
        })
}

fn get_distance_from_line((a_x, a_y): (i32, i32), (b_x, b_y): (i32, i32), (c_x, c_y): (i32, i32)) -> f64 {
    if a_x == b_x {
        return (c_x - a_x).abs() as f64;
    }
    let m = (a_y - b_y) as f64 / (a_x - b_x) as f64;
    let n = (a_y as f64) - m * (a_x as f64);

    (-m * (c_x as f64) + (c_y as f64) - n) / (m.powi(2) + 1.0)
}
