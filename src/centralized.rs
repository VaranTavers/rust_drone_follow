use opencv as cv;
use cv::core::*;

use crate::traits::PointSystem;

pub struct Centralized {
    width: usize,
    height: usize,
}

impl Centralized {
    pub fn new(width: usize, height: usize) -> Centralized {
        Centralized {
            width,
            height,
        }
    }
}

impl PointSystem for Centralized {
    /// Returns the center of this descartes coordinate-system (0, 0)
    fn get_center(&self) -> Point {
        Point::new(0, 0)
    }

    /// Takes a point from the coordinate system of an image and returns one in this descartes coordinate-system
    fn from_image_coords(&self, point: &Point) -> Point {
        Point::new(
                point.x - (self.width as i32 / 2),
                (point.y - (self.height as i32 / 2)) * (-1)
            )
    }

    /// Takes a point from this descartes coordinate-system and returns one in the coordinate system of an image
    fn to_image_coords(&self, point: &Point) -> Point {
        Point::new(
                point.x + (self.width as i32 / 2),
                (self.height as i32 / 2) - point.y
            )
    }
}

#[cfg(test)]
mod tests {
    use super::Centralized;
    use crate::traits::PointSystem;
    use opencv::core::Point;

    #[test]
    fn center_x_should_be_zero() {
        let sut = Centralized::new(640, 480);

        assert_eq!(sut.get_center().x, 0)
    }

    #[test]
    fn center_y_should_be_zero() {
        let sut = Centralized::new(640, 480);

        assert_eq!(sut.get_center().y, 0)
    }

    #[test]
    fn point_should_map_to_m320_240() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(0, 0);
        let new_point = sut.from_image_coords(&point);

        assert!(new_point.x == -320 && new_point.y == 240)
    }

    #[test]
    fn point_should_map_to_320_m240() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(640, 480);
        let new_point = sut.from_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == -240)
    }

    #[test]
    fn point_should_map_to_m320_m240() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(0, 480);
        let new_point = sut.from_image_coords(&point);

        assert!(new_point.x == -320 && new_point.y == -240)
    }

    #[test]
    fn point_should_map_to_320_240() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(640, 0);
        let new_point = sut.from_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == 240)
    }

    #[test]
    fn point_should_map_to_0_0() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(-320, 240);
        let new_point = sut.to_image_coords(&point);

        assert!(new_point.x == 0 && new_point.y == 0)
    }

    #[test]
    fn point_should_map_to_640_480() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(320, -240);
        let new_point = sut.to_image_coords(&point);

        assert!(new_point.x == 640 && new_point.y == 480)
    }

    #[test]
    fn point_should_map_to_0_480() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(-320, -240);
        let new_point = sut.to_image_coords(&point);

        assert!(new_point.x == 0 && new_point.y == 480)
    }

    #[test]
    fn point_should_map_to_640_0() {
        let sut = Centralized::new(640, 480);
        let point = Point::new(320, 240);
        let new_point = sut.to_image_coords(&point);

        assert!(new_point.x == 640 && new_point.y == 0)
    }
}