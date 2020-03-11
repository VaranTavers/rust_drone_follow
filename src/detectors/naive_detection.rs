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
    /// Debug
    tmp_points: (i32, i32, i32, i32),
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
            tmp_points: (0, 0, 0, 0),
        }
    }

    fn get_point_from_contours(&mut self, contours: &cv::types::VectorOfVectorOfPoint, lower_size: f64) -> (Option<Point>, u32, TanableAngle) {
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
                            .fold((0, 0), |(ax, ay), c_p| (ax + c_p.x, ay + c_p.y));
                        let l = contour.len() as i32;
                        let (c_x, c_y) = (s_x / l, s_y / l);

                        println!("center ({}, {})", c_x, c_y);
                        return (Some(Point::new(c_x, c_y)), 1, self.get_angle((c_x, c_y), contour));
                    }
                    _ => {

                    }
                }
            }
        }

        (None, 0, TanableAngle::Angle(0.0))
    }

    fn get_angle(&mut self, (c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> TanableAngle {
        let ((a_x, a_y), (b_x, b_y)) = get_points_from_two_sides((c_x, c_y), contour);

        self.tmp_points = (a_x, a_y, b_x, b_y);
        if a_y == b_y {
            return TanableAngle::Vertical;
        }

        let m = (a_y - b_y) as f64 / (a_x - b_x) as f64;
        TanableAngle::Angle((-1.0 / m).atan())
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

        let (point, cert, angle) = self.get_point_from_contours(&contours, self.lower_size);

        self.point = point;
        self.cert = cert as f64;
        self.angle = angle;
    }

    fn draw_on_image(&self, img: &mut Mat) {
        let k = 100;
        match self.point {
            Some(p) => {
                let c_point = p.clone();
                let other_point;
                match self.angle {
                    TanableAngle::Angle(angle) => {
                        let tan_angle = angle.tan();
                        other_point = Point::new(c_point.x + k, c_point.y + (k as f64 * tan_angle) as i32);
                    }
                    TanableAngle::Vertical => {
                        other_point = Point::new(c_point.x, c_point.y - k);
                    }
                }

                line(img,
                     c_point,
                     other_point,
                     Scalar::new(0.0, 0.0, 255.0, 255.0),
                     2, LINE_8, 0).unwrap();

                let (a_x, a_y, b_x, b_y) = self.tmp_points;
                circle(img,
                       Point::new(a_x, a_y),
                       5,
                       Scalar::new(0.0, 100.0, 0.0, 255.0),
                       2,
                       LINE_8,
                       0).unwrap();
                circle(img,
                       Point::new(b_x, b_y),
                       5,
                       Scalar::new(0.0, 255.0, 0.0, 255.0),
                       2,
                       LINE_8,
                       0).unwrap();
            }
            None => {
            }
        }
    }
}

/// Gets the two closest points to the center of the hat, which are at least a constant far away from
/// eachother.
/// This is done by finding the closest point (A) and then finding the second point (B) so that
/// the center point (C) is the closest to the AB line
fn get_points_from_two_sides((c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> ((i32, i32), (i32, i32)) {
    let (cl_x, cl_y, d) = get_closest_point_to_center_from_contour((c_x, c_y), contour);

    let (other_x, other_y, d) = contour.iter()
        .fold((0, 0, 500000.0), |(dx, dy, d), c_p| {
            let dist = get_distance_from_line((cl_x, cl_y), (c_p.x, c_p.y), (c_x, c_y));
            if dist < d {
                return (c_p.x, c_p.y, dist);
            }
            (dx, dy, d)
        });

    if d > 1000.0 {
        println!("a ({}, {}), b ({}, {}), c ({}, {}), d = {}", cl_x, cl_y, other_x, other_y, c_x, c_y, d);
        cv::highgui::wait_key(100000);
    }
    println!("d: {}", d);
    ((cl_x, cl_y), (other_x, other_y))
}

fn get_closest_point_to_center_from_contour((c_x, c_y): (i32, i32), contour: &VectorOfPoint) -> (i32, i32, i32) {
    contour.iter()
        .fold((0, 0, 5000000), |(dx, dy, d), c_p| {
            let dist_x = c_x - c_p.x;
            let dist_y = c_y - c_p.y;
            let dist = (dist_x.pow(2) + dist_y.pow(2));
            if dist < d {
                return (c_p.x, c_p.y, dist);
            }
            (dx, dy, d)
        })
}

fn get_distance_from_line((a_x, a_y): (i32, i32), (b_x, b_y): (i32, i32), (c_x, c_y): (i32, i32)) -> f64 {
    let a = (a_y - b_y);
    let b = (a_x - b_x);
    let c = b_x * a_y + b_y * a_x;

    (a * c_x - b * c_y + c).abs() as f64 / ((a.pow(2) + b.pow(2)) as f64).sqrt()
}

