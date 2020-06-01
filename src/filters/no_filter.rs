use crate::traits::{Filter};

use crate::models::geometric_point::GeometricPoint;

use crate::utils::marker_drawer::MarkerDrawer;
use crate::utils::opencv_custom::get_blue;

/// This is a filler Filter it doesn't calculate any new data (except for vx, vy) and doesn't modify
/// old data in any meaningful way. It passes forward the data it has got from the detector.
pub struct NoFilter {
    point: Option<GeometricPoint>,
    angle: f64,
    vx: f64,
    vy: f64,
    cert: f64,
}

impl NoFilter {
    /// NoFilter doesn't need any parameters since it doesn't do any calculations.
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
    /// Simply copies the estimation, that it got from the detector. vx and vy are calculated as
    /// a difference of old point and the new point.
    fn update_estimation(&mut self, point: Option<GeometricPoint>, angle: Option<f64>, cert: f64) {
        match &self.point {
            Some(p) => {
                if let Some(point) = &point {
                    self.vx = (point.x - p.x) as f64;
                    self.vy = (point.y - p.y) as f64;
                }
            }
            _ => { }
        }
        self.point = point;
        if let Some(angle) = angle {
            self.angle = angle;
        }
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

    fn draw_on_image(&self, m_d: &mut MarkerDrawer) {
        if let Some(p) = &self.point {
            m_d.point(p, get_blue());
        }
    }
}
