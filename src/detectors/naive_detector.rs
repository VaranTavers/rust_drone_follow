use opencv as cv;
use cv::core::*;
use opencv::imgproc::{contour_area};
use opencv::types::{VectorOfPoint};

use crate::opencv_custom::{get_contours, get_red, get_green};
use crate::traits::{Detector};
use crate::geometric_point::{GeometricPoint, get_center_of_geometric_points, get_closest_from_geometric_points_to_point};
use crate::point_converter::PointConverter;
use crate::hat::Hat;
use crate::marker_drawer::MarkerDrawer;

const PI: f64 = std::f64::consts::PI;

enum TanableAngle {
    Vertical,
    Angle(f64)
}

/// This is the most basic detection this library offers. It basically searches for the things in
/// the given color range, closest in size to the given size (with a maximum difference of 50%)
/// and it calculates it's central point by averaging all the points of the contour of the object
/// the angle by calculating it's sides and calculating the line's, which connects them, normal.
///
/// This detector works with baseball caps, and by angle we mean the angle between a line which goes
/// intersect's the hat for it's longest section and a horizontal line.
///
/// This angle will always be between -pi/2 and pi/2.
///
/// This Detector doesn't take into account previous coordinates of the tracked object.
pub struct NaiveDetector {
    point: Option<GeometricPoint>,
    cert: f64,
    angle: TanableAngle,
    hat: Hat,
    /// Debug
    hat_side_points: (GeometricPoint, GeometricPoint),
}

impl NaiveDetector {
    /// Requires a Hat given to it, which contains the information about the hat that the detector
    /// is looking for.
    ///
    /// Usage:
    ///
    ///```
    /// use rust_drone_follow::detectors::naive_detector::NaiveDetector;
    /// use rust_drone_follow::opencv_custom::LabColor;
    /// use rust_drone_follow::hat::Hat;
    /// // ...
    /// # fn main() {
    ///     let hat = Hat::new(
    ///            LabColor::new(0, 20, -127),
    ///            LabColor::new(80, 127, -20),
    ///            1200.0
    ///        );
    ///     let naive_detector = NaiveDetector::new(hat);
    /// # }
    /// ```
    pub fn new(hat: Hat) -> NaiveDetector {
        NaiveDetector {
            point: None,
            cert: 0.0,
            angle: TanableAngle::Angle(0.0),
            hat_side_points: (GeometricPoint::new(0, 0), GeometricPoint::new(0, 0)),
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

impl Detector for NaiveDetector {
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

    /// Call this for every frame you want to use the detector for. It recalculates the position,
    /// angle and certainty.
    fn detect_new_position(&mut self, img: &Mat, _old_pos: Option<Point>, p_c: &PointConverter) {
        let contours = get_contours(img, &self.hat.color_low, &self.hat.color_high);
        let contour_option = get_best_fit_contour(&contours, self.hat.size_avg);

        match contour_option {
            Some((contour, cert)) => {
                let contour_cent = contour
                    .iter()
                    .map(|p| p_c.convert_from_image_coords(p))
                    .collect::<Vec<GeometricPoint>>();

                let center = get_center_of_geometric_points(&contour_cent);

                self.cert = cert;
                self.angle = self.get_angle(&center, &contour_cent);
                self.point = Some(center);
            }
            None => {
                self.cert = 0.0;
                self.point = None;
            }
        }
    }

    /// Call this only if you want to visualize the detected points, and the angle.
    fn draw_on_image(&self, m_d: &mut MarkerDrawer) {
        let k = 100;
        match &self.point {
            Some(p) => {
                let other_point = match self.angle {
                    TanableAngle::Angle(angle) => {
                        GeometricPoint::new(p.x + k, p.y + (k as f64 * angle.tan()) as i32)
                    }
                    TanableAngle::Vertical => {
                        GeometricPoint::new(p.x, p.y - k)
                    }
                };

                m_d.line(p, &other_point, get_red());

                let (cgp, ogp) = &self.hat_side_points;
                m_d.point(cgp, Scalar::new(0.0, 100.0, 0.0, 255.0));
                m_d.line(cgp, ogp, get_green());
                m_d.point(ogp, get_green());
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
    let (closest_point, _d) = get_closest_from_geometric_points_to_point(contour,center_point);
    let symmetric_to_center = GeometricPoint::new(center_point.x * 2 - closest_point.x, center_point.y * 2 - closest_point.y);
    let (other_point, _d) = get_closest_from_geometric_points_to_point(contour, &symmetric_to_center);

    (closest_point, other_point)
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
        (l, (500.0 / best_fit_area_diff).min(1.0).max(0.0))
    })
}

/// The area can only differ with a maximum of 50%
fn get_contour_with_closest_area_to(c_with_area: &Vec<(f64, VectorOfPoint)>, size_avg: f64) -> (f64, f64, Option<&VectorOfPoint>) {
    c_with_area.iter()
        .fold((-1.0, 500000.0, None), |(acc_a, acc_a_diff, acc_c), (c_a, c_c)| {
            if *c_a >= size_avg / 2.0 && *c_a <= size_avg * 1.5 && (size_avg - *c_a).abs() < acc_a_diff {
                return (*c_a, (size_avg - *c_a).abs(), Some(c_c));
            }
            (acc_a, acc_a_diff, acc_c)
        })
}

