use opencv as cv;

use cv::core::*;
use crate::traits::{Filter, PointSystem};
use crate::geometric_point::GeometricPoint;

pub struct NoFilter<P: PointSystem> {
    point_system: P,
    point: Option<GeometricPoint>,

}

impl<P: PointSystem> NoFilter<P> {
    pub fn new(point_system: P) -> NoFilter<P> {
       NoFilter {
            point_system: point_system,
            point: Some(GeometricPoint::new(0, 0)),
       }
    }
}

impl<P: PointSystem> Filter for NoFilter<P> {
    fn estimate_new_position(&mut self, point: &GeometricPoint) {
        self.point = Some((*point).clone());
    }
    
    fn get_estimated_position(&self) -> Option<GeometricPoint> {
        self.point.as_ref().map(|p| p.clone())
    }

    fn get_estimated_position_for_detector(&self) -> Option<Point> {
        self.point.as_ref().map(|p| self.point_system.convert_to_image_coords(&p))
    }

    fn get_estimated_certainty(&self) -> f64 {
        1.0
    }

    fn draw_on_image(&self, _img: &mut Mat) {
        
    }

    fn get_difference_vector(&self) -> Option<GeometricPoint> {
        let center = self.point_system.get_center();
        self.point.as_ref().map(|p| GeometricPoint::new(p.x - center.x, p.y - center.y))
    }

}
