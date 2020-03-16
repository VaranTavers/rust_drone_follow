use opencv as cv;
use cv::core::*;
use cv::prelude::*;
use opencv::imgproc::{contour_area, line, LINE_8, circle};

use crate::opencv_custom::{MyColor, get_contours, GeometricPoint};
use crate::traits::{Detector, PointSystem};
use crate::point_systems::centralized::Centralized;

const PI: f64 = std::f64::consts::PI;

enum TanableAngle {
    Vertical,
    Angle(f64)
}

pub struct NaiveDetection {
    lower_color: MyColor,
    upper_color: MyColor,
    lower_size: f64,
    point: Option<GeometricPoint>,
    cert: f64,
    angle: TanableAngle,
    /// Debug
    tmp_points: (Point, Point),
    cent: Centralized,
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
            tmp_points: (Point::new(0, 0), Point::new(0, 0)),
            cent: Centralized::new(640, 368),
        }
    }

    fn get_angle(&mut self, center_point: &GeometricPoint, contour: &Vec<GeometricPoint>) -> TanableAngle {
        let (a, b) = get_points_from_two_sides(center_point, contour);

        let p_a = self.cent.convert_to_image_coords(&a);
        let p_b = self.cent.convert_to_image_coords(&b);
        self.tmp_points = (p_a, p_b);
        if a.y == b.y {
            return TanableAngle::Vertical;
        }

        let m = (a.y - b.y) as f64 / (a.x - b.x) as f64;
        TanableAngle::Angle((1.0 / m).atan())
    }
}

impl Detector for NaiveDetection {
    fn get_detected_position(&self) -> Option<GeometricPoint> {
        self.point.as_ref().map(|a| a.clone())
    }

    fn get_detected_angle(&self) -> Option<f64> {
        match &self.angle {
            TanableAngle::Angle(i) => {
                Some(*i)
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

        let contour_option = get_biggest_contour(&contours, self.lower_size);

        match contour_option {
            Some(contour) => {
                let contour_cent = contour
                    .iter()
                    .map(|p| self.cent.convert_from_image_coords(p))
                    .collect::<Vec<GeometricPoint>>();

                let (s_x, s_y) = contour_cent
                    .iter()
                    .fold((0, 0), |(a_x, a_y), p| (a_x + p.x, a_y + p.y));
                let l = contour.len();
                let (c_x, c_y) = (s_x / (l as i32), s_y / (l as i32));

                let angle= self.get_angle(&GeometricPoint::new(c_x, c_y), &contour_cent);

                self.point = Some(GeometricPoint::new(c_x, c_y));
                self.cert = 1.0;
                self.angle = angle;
            }
            None => {
                self.point = None;
                self.cert = 1.0;
                self.angle = TanableAngle::Angle(0.0);
            }
        }
    }

    fn draw_on_image(&self, img: &mut Mat) {
        let k = 100;
        match &self.point {
            Some(p) => {
                let c_point = self.cent.convert_to_image_coords(&p);
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

                let (closest, other) = self.tmp_points;
                circle(img,
                       closest.clone(),
                       5,
                       Scalar::new(0.0, 100.0, 0.0, 255.0),
                       2,
                       LINE_8,
                       0).unwrap();
                line(img,
                     closest.clone(),
                     other.clone(),
                     Scalar::new(0.0, 255.0, 0.0, 255.0),
                     2, LINE_8, 0).unwrap();
                circle(img,
                       other.clone(),
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
fn get_points_from_two_sides(center_point: &GeometricPoint, contour: &Vec<GeometricPoint>) -> (GeometricPoint, GeometricPoint) {
    let (closest_point, _d) = get_closest_point_to_center_from_contour(center_point, contour);

    let (other_point, d) = contour.iter()
        .fold((GeometricPoint::new(0, 0), 500000.0), |(other_point, d), current_point| {
            let dist = get_distance_from_line(&closest_point, current_point, center_point);
            if dist < d {
                return (current_point.clone(), dist);
            }
            (other_point, d)
        });

    if d > 10.0 {
        println!("a ({}, {}), b ({}, {}), c ({}, {}), d = {}", closest_point.x, closest_point.y, other_point.x, other_point.y, center_point.x, center_point.y, d);
        cv::highgui::wait_key(100000);
    }
    println!("d: {}, {} {}", d, closest_point.x, closest_point.y);
    (closest_point, other_point)
}

fn get_closest_point_to_center_from_contour(c: &GeometricPoint, contour: &Vec<GeometricPoint>) -> (GeometricPoint, i32) {
    contour.iter()
        .fold((GeometricPoint::new(0, 0), 5000000), |(p, d), c_p| {
            let dist_x = c.x - c_p.x;
            let dist_y = c.y - c_p.y;
            let dist_sq = dist_x.pow(2) + dist_y.pow(2);
            if dist_sq < d {
                return (c_p.clone(), dist_sq);
            }
            (p, d)
        })
}

fn get_distance_from_line(a: &GeometricPoint, b: &GeometricPoint, c: &GeometricPoint) -> f64 {
    let eq_a = a.y - b.y;
    let eq_b = a.x - b.x;
    let eq_c = b.x * a.y - b.y * a.x;

    (eq_a * c.x - eq_b * c.y + eq_c).abs() as f64 / ((eq_a.pow(2) + eq_b.pow(2)) as f64).sqrt()
}

fn get_biggest_contour(contours: &cv::types::VectorOfVectorOfPoint, lower_size: f64) -> Option<Vec<Point>> {
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
                    let mut ret = Vec::new();
                    let _ = contour.iter()
                        .fold(0, |_, c_p| {
                            ret.push(Point::new(c_p.x, c_p.y));
                            0
                        });
                    return Some(ret);
                }
                _ => {

                }
            }
        }
    }

    None
}
