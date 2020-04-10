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
    // If you need to use this in your own code, you can instantiate one with new()
    pub fn new() -> MarkerDrawer {
        MarkerDrawer {
            markers: Vec::new(),
        }
    }
    // Draws a small circle with radius of 5 and thickness 2, when draw on image is called
    pub fn point(&mut self, point: &GeometricPoint, color: Scalar) {
        self.markers.push(Marker::Point(point.clone(), color));
    }

    // Draws a line with thickness 1, between the two given points when draw on image is called
    pub fn line(&mut self, point1: &GeometricPoint, point2: &GeometricPoint, color: Scalar) {
        self.markers.push(Marker::Line(point1.clone(), point2.clone(), color));
    }

    // Draws a small circle with the given radius and thickness 1, when draw on image is called
    pub fn circle(&mut self, point: &GeometricPoint, radius: i32, color: Scalar) {
        self.markers.push(Marker::Circle(point.clone(), radius, color));
    }

    // Draws the saved Markers on the given image, requires a PointConverter.
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