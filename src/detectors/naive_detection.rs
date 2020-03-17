use opencv as cv;
use cv::core::*;
use cv::prelude::*;
use opencv::imgproc::{contour_area, line, LINE_8, circle};

use crate::opencv_custom::{MyColor, get_contours, GeometricPoint, line_c, get_red, get_green};
use crate::traits::{Detector, PointSystem};
use crate::point_systems::centralized::Centralized;
use crate::hat::Hat;
use opencv::types::{VectorOfVectorOfPoint, VectorOfPoint};

const PI: f64 = std::f64::consts::PI;

enum TanableAngle {
    Vertical,
    Angle(f64)
}

pub struct NaiveDetection {
    point: Option<GeometricPoint>,
    cert: f64,
    angle: TanableAngle,
    hat: Hat,
    /// Debug
    hat_side_points: (GeometricPoint, GeometricPoint),
    cent: Centralized,
}

impl NaiveDetection {
    pub fn new(hat: Hat, cent: Centralized) -> NaiveDetection {
        NaiveDetection {
            point: None,
            cert: 0.0,
            angle: TanableAngle::Angle(0.0),
            hat_side_points: (GeometricPoint::new(0, 0), GeometricPoint::new(0, 0)),
            cent,
            hat
        }
    }

    fn get_angle(&mut self, center_point: &GeometricPoint, contour: &Vec<GeometricPoint>) -> TanableAngle {
        let (a, b) = get_points_from_two_sides(center_point, contour);
        let d = a.y - b.y;
        let m = d as f64 / (a.x - b.x) as f64;

        // Debug
        self.hat_side_points = (a, b);
        if d == 0 {
            return TanableAngle::Vertical;
        }

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
        let contours = get_contours(img, &self.hat.color_low, &self.hat.color_high);
        let contour_option = get_best_fit_contour(&contours, self.hat.size_avg);

        match contour_option {
            Some((contour, cert)) => {
                let contour_cent = contour
                    .iter()
                    .map(|p| self.cent.convert_from_image_coords(p))
                    .collect::<Vec<GeometricPoint>>();

                let center = get_center_of_contour(&contour_cent);

                self.cert = cert;
                self.angle = self.get_angle(&center, &contour_cent);
                self.point = Some(center);
            }
            None => {
                self.cert = 0.0;
            }
        }
    }

    fn draw_on_image(&self, img: &mut Mat) {
        let k = 100;
        match &self.point {
            Some(p) => {
                let c_point = self.cent.convert_to_image_coords(&p);
                let other_point = match self.angle {
                    TanableAngle::Angle(angle) => {
                        Point::new(c_point.x + k, c_point.y + (k as f64 * angle.tan()) as i32)
                    }
                    TanableAngle::Vertical => {
                        Point::new(c_point.x, c_point.y - k)
                    }
                };

                line_c(img,&c_point, &other_point, get_red());

                let (cgp, ogp) = &self.hat_side_points;
                let (closest, other) = (self.cent.convert_to_image_coords(cgp), self.cent.convert_to_image_coords(ogp));
                circle(img, closest.clone(), 5, Scalar::new(0.0, 100.0, 0.0, 255.0), 2, LINE_8, 0).unwrap();
                line_c(img, &closest, &other, get_green());
                circle(img,
                       other.clone(),
                       5,
                       get_green(),
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
/// This is done by finding the closest point (A) then calculating it's symmetric (A') in regards of the
/// center point (C), and then finding the closest point to A'.
fn get_points_from_two_sides(center_point: &GeometricPoint, contour: &Vec<GeometricPoint>) -> (GeometricPoint, GeometricPoint) {
    let (closest_point, _d) = get_closest_point_to_center_from_contour(center_point, contour);
    let symmetric_to_center = GeometricPoint::new(center_point.x * 2 - closest_point.x, center_point.y * 2 - closest_point.y);
    let (other_point, d) = get_closest_point_to_center_from_contour(&symmetric_to_center, contour);

    (closest_point, other_point)
}

fn get_closest_point_to_center_from_contour(c: &GeometricPoint, contour: &Vec<GeometricPoint>) -> (GeometricPoint, i32) {
    contour.iter()
        .fold((GeometricPoint::new(0, 0), 5000000), |(p, d), c_p| {
            let dist_sq = (c.x - c_p.x).pow(2) + (c.y - c_p.y).pow(2);
            if dist_sq < d {
                return (c_p.clone(), dist_sq);
            }
            (p, d)
        })
}

fn get_best_fit_contour(contours: &cv::types::VectorOfVectorOfPoint, size_avg: f64) -> Option<(Vec<Point>, f64)> {
    let c_with_area = contours.iter()
        .map(|contour| (contour_area(&contour, false).unwrap(), contour))
        .collect::<Vec<(f64, VectorOfPoint)>>();
    let (_, best_fit_area_diff, best_fit) = get_contour_with_closest_area_to(&c_with_area, size_avg);

    best_fit.map(|contour| {
        let l = contour.iter()
            .map(|p| Point::new(p.x, p.y))
            .collect::<Vec<Point>>();
        (l, (500.0 / best_fit_area_diff).min(1.0))
    })
}

fn get_contour_with_closest_area_to(c_with_area: &Vec<(f64, VectorOfPoint)>, size_avg: f64) -> (f64, f64, Option<&VectorOfPoint>) {
    c_with_area.iter()
        .fold((-1.0, 500000.0, None), |(acc_a, acc_a_diff, acc_c), (c_a, c_c)| {
            if (size_avg - *c_a).abs() < acc_a_diff {
                return (*c_a, (size_avg - *c_a).abs(), Some(c_c));
            }
            (acc_a, acc_a_diff, acc_c)
        })
}

fn get_center_of_contour(contour: &Vec<GeometricPoint>) -> GeometricPoint {
    let (s_x, s_y) = contour_cent
        .iter()
        .fold((0, 0), |(a_x, a_y), p| (a_x + p.x, a_y + p.y));
    let l = contour.len();
    GeometricPoint::new(s_x / (l as i32), s_y / (l as i32))
}
