use opencv as cv;
use cv::core::*;

use crate::geometric_point::GeometricPoint;

pub struct PointConverter {
    width: usize,
    height: usize,
}

impl PointConverter {
    pub fn new(width: usize, height: usize) -> PointConverter {
        PointConverter {
            width,
            height,
        }
    }
    /// Returns the center of this descartes coordinate-system (0, 0)
    pub fn get_center(&self) -> GeometricPoint {
        GeometricPoint::new(0, 0)
    }

    /// Takes a point from the coordinate system of an image and returns one in this descartes coordinate-system
    pub fn convert_from_image_coords(&self, point: &Point) -> GeometricPoint {
        GeometricPoint::new(
                point.x - (self.width as i32 / 2),
                (point.y - (self.height as i32 / 2)) * (-1)
            )
    }

    /// Takes a point from this descartes coordinate-system and returns one in the coordinate system of an image
    pub fn convert_to_image_coords(&self, point: &GeometricPoint) -> Point {
        Point::new(
                point.x + (self.width as i32 / 2),
                (self.height as i32 / 2) - point.y
            )
    }
}

#[cfg(test)]
mod tests {
    use super::PointConverter;
    use opencv::core::Point;
    use crate::geometric_point::GeometricPoint;

    #[test]
    fn center_x_should_be_zero() {
        let sut = PointConverter::new(640, 480);

        assert_eq!(sut.get_center().x, 0)
    }

    #[test]
    fn center_y_should_be_zero() {
        let sut = PointConverter::new(640, 480);

        assert_eq!(sut.get_center().y, 0)
    }

    #[test]
    fn point_should_map_to_m320_240() {
        let sut = PointConverter::new(640, 480);
        let point = Point::new(0, 0);
        let new_point = sut.convert_from_image_coords(&point);

        assert!(new_point.x == -320 && new_point.y == 240)
    }

    #[test]
    fn point_should_map_to_320_m240() {
        let sut = PointConverter::new(640, 480);
        let point = Point::new(640, 480);
        let new_point = sut.convert_from_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == -240)
    }

    #[test]
    fn point_should_map_to_m320_m240() {
        let sut = PointConverter::new(640, 480);
        let point = Point::new(0, 480);
        let new_point = sut.convert_from_image_coords(&point);

        assert!(new_point.x == -320 && new_point.y == -240)
    }

    #[test]
    fn point_should_map_to_320_240() {
        let sut = PointConverter::new(640, 480);
        let point = Point::new(640, 0);
        let new_point = sut.convert_from_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == 240)
    }

    #[test]
    fn point_should_map_to_0_0() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(-320, 240);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 0 && new_point.y == 0)
    }

    #[test]
    fn point_should_map_to_640_480() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(320, -240);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 640 && new_point.y == 480)
    }

    #[test]
    fn point_should_map_to_0_480() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(-320, -240);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 0 && new_point.y == 480)
    }

    #[test]
    fn point_should_map_to_640_0() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(320, 240);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 640 && new_point.y == 0)
    }

    #[test]
    fn point_should_map_to_320_239() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(0, 1);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == 239)
    }

    #[test]
    fn point_should_map_to_320_241() {
        let sut = PointConverter::new(640, 480);
        let point = GeometricPoint::new(0, -1);
        let new_point = sut.convert_to_image_coords(&point);

        assert!(new_point.x == 320 && new_point.y == 241)
    }
}