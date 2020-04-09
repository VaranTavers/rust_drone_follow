use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;

use opencv::core::{Scalar, Mat};
use opencv::imgproc::{circle, LINE_8, line};

enum Marker {
    Point(GeometricPoint, Scalar),
    Line(GeometricPoint, GeometricPoint, Scalar),
    Circle(GeometricPoint, i32, Scalar)
}

pub struct MarkerDrawer {
    markers: Vec<Marker>,
}

impl MarkerDrawer {
    pub fn new() -> MarkerDrawer {
        MarkerDrawer {
            markers: Vec::new(),
        }
    }
    pub fn point(&mut self, point: &GeometricPoint, color: Scalar) {
        self.markers.push(Marker::Point(point.clone(), color));
    }

    pub fn line(&mut self, point1: &GeometricPoint, point2: &GeometricPoint, color: Scalar) {
        self.markers.push(Marker::Line(point1.clone(), point2.clone(), color));
    }

    pub fn circle(&mut self, point: &GeometricPoint, radius: i32, color: Scalar) {
        self.markers.push(Marker::Circle(point.clone(), radius, color));
    }

    pub fn draw_on_image(&mut self, img: &mut Mat, p_c: &PointConverter) {
        for marker in self.markers.iter() {
            match marker {
                Marker::Point(p, color) => {
                    circle(img,
                           p_c.convert_to_image_coords(p),
                           5,
                           color.clone(),
                           2,
                           LINE_8,
                           0).unwrap();
                }
                Marker::Line(p1, p2, color) => {
                    line(img,
                         p_c.convert_to_image_coords(p1),
                         p_c.convert_to_image_coords(p2),
                         color.clone(),
                         1,
                         LINE_8,
                         0
                    ).unwrap();
                }
                Marker::Circle(p, radius, color) => {
                    circle(img,
                           p_c.convert_to_image_coords(p),
                           *radius,
                           color.clone(),
                           2,
                           LINE_8,
                           0).unwrap();
                }
            }
        }
        self.markers = Vec::new();
    }
}