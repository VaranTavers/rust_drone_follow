use opencv as cv;

use cv::core::*;
use crate::traits::{Filter, PointSystem};

pub struct NoFilter<P: PointSystem> {
    point_system: P,
    point: Option<Point>,

}

impl<P: PointSystem> NoFilter<P> {
    pub fn new(point_system: P) -> NoFilter<P> {
       NoFilter {
            point_system: point_system,
            point: Some(Point::new(0, 0)),
       }
    }
}

impl<P: PointSystem> Filter for NoFilter<P> {
    fn estimate_new_position(&mut self, point: &Point) {
        self.point = Some(self.point_system.from_image_coords(point));
    }
    
    fn get_estimated_position(&self) -> Option<Point> {
        self.point.clone()
    }

    fn get_estimated_position_for_detector(&self) -> Option<Point> {
            self.point.map(|p| self.point_system.to_image_coords(&p))
    }

    fn get_estimated_certainty(&self) -> f64 {
        1.0
    }

    fn draw_on_image(&self, _img: &mut Mat) {
        
    }

    fn get_difference_vector(&self) -> Option<Point> {
        let center = self.point_system.get_center();
        self.point.map(|p| Point::new(p.x - center.x, p.y - center.y))
    }

}
