use opencv as cv;

use cv::core::*;
use crate::traits::{Filter};
use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;

pub struct NoFilter {
    point: Option<GeometricPoint>,
    angle: f64,
    vx: f64,
    vy: f64,
    cert: f64,
}

impl NoFilter {
    pub fn new() -> NoFilter {
       NoFilter {
           point: None,
           angle: 0.0,
           cert: 0.0,
           vx: 0.0,
           vy: 0.0,
       }
    }
}

impl Filter for NoFilter {
    /// Simply copies the estimation, that it got from the detector
    fn update_estimation(&mut self, point: &GeometricPoint, angle: f64, cert: f64) {
        match &self.point {
            Some(p) => {
                self.vx = (point.x - p.x) as f64;
                self.vy = (point.y - p.y) as f64;
            }
            _ => { }
        }
        self.point = Some((*point).clone());
        self.angle = angle;
        self.cert = cert;
    }

    fn get_estimated_position(&self) -> Option<GeometricPoint> {
        self.point.as_ref().map(|p| p.clone())
    }

    fn get_estimated_angle(&self) -> f64 {
        self.angle
    }

    fn get_estimated_vx(&self) -> f64 {
        self.vx
    }

    fn get_estimated_vy(&self) -> f64 {
        self.vy
    }

    fn get_estimation_certainty(&self) -> f64 {
        self.cert
    }

    fn draw_on_image(&self, _img: &mut Mat, _p_c: &PointConverter) {
        
    }
}
