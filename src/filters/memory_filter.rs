use opencv as cv;

use cv::core::*;
use crate::traits::{Filter};
use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;
use crate::marker_drawer::MarkerDrawer;
use crate::opencv_custom::get_blue;

/// Same as NoFilter, but retains last known position of the hat.
pub struct MemoryFilter {
    frames_unknown: usize,
    max_frames_unknown: usize,
    point: Option<GeometricPoint>,
    angle: f64,
    vx: f64,
    vy: f64,
    cert: f64,
}

impl MemoryFilter {
    /// MemoryFilter takes as a parameter the number of frames after which it forgets the last
    /// known position.
    pub fn new(max_frames_unknown: usize) -> MemoryFilter {
       MemoryFilter {
           frames_unknown: 0,
           max_frames_unknown,
           point: None,
           angle: 0.0,
           cert: 0.0,
           vx: 0.0,
           vy: 0.0,
       }
    }
}

impl Filter for MemoryFilter {
    /// Simply copies the estimation, that it got from the detector. vx and vy are calculated as
    /// a difference of old point and the new point. If there is no new detection, retains the old
    /// one, until given amount of frames.
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
        match point {
            Some(p) => {
                self.frames_unknown = 0;
                self.point = Some(p);
            }
            None => {
                if self.frames_unknown == self.max_frames_unknown {
                    self.point = None;
                    self.frames_unknown = 0;
                }
                self.frames_unknown += 1;
            }
        }
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
