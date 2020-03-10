use opencv::types::{VectorOfVec4f};
use opencv::core::{Point, Mat, Scalar, no_array};
use opencv::types::VectorOfVectorOfPoint;
use opencv::highgui::imshow;

use crate::image_processing::*;
use opencv::imgproc::{draw_contours, LINE_8};

#[derive(Clone)]
pub struct LineEquation {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub p1: Point,
    pub p2: Point,
}

impl LineEquation {
    pub fn from_vec_of_vec4f(line_vector: &VectorOfVec4f) -> Vec<LineEquation> {
        let mut ret_vec = Vec::new();
        for line_item in line_vector.iter() {
            let x1 = *line_item.get(0).unwrap() as i32;
            let y1 = *line_item.get(1).unwrap() as i32;
            let x2 = *line_item.get(2).unwrap() as i32;
            let y2 = *line_item.get(3).unwrap() as i32;
            let m = (y2 - y1) as f64 / (x2 - x1) as f64;

            ret_vec.push(LineEquation {
                a: -m,
                b: 1.0,
                c: m * (x1 as f64) - (y1 as f64),
                p1: Point::new(x1, y1),
                p2: Point::new(x2, y2)
            })
        }
        ret_vec
    }
}

pub fn do_processing(img: &Mat, contours: &VectorOfVectorOfPoint) -> Vec<LineEquation> {
    let mut new_img = mat_size_of_other(img);
    draw_contours(&mut new_img,
                    contours,
                    -1,
                    Scalar::new(255.0, 255.0, 255.0, 255.0),
                    2,
                    LINE_8,
                    &no_array().unwrap(),
                    0,
                    Point::new(0,0)
    ).unwrap();
    let graymane = grayscale(&new_img);
    let blurred = gaussian_blur_phase(&graymane);
    let cannied = canny_phase(&blurred);
    let canny_blur = gaussian_blur_phase(&cannied);

    imshow("CB", &canny_blur);

    let cpp_lines = hough_phase(&canny_blur);

    LineEquation::from_vec_of_vec4f(&cpp_lines)
}

fn center_and_flip(width: usize, height: usize, line_eq: &LineEquation) -> LineEquation {
    let min_x = -(width as i32) / 2;
    let min_y = -(height as i32) / 2;
    let p1 = Point::new(line_eq.p1.x + min_x, (line_eq.p1.y + min_y) * -1);
    let p2 = Point::new(line_eq.p2.x + min_x, (line_eq.p2.y + min_y) * -1);

    LineEquation {
        a: line_eq.a,
        b: line_eq.b,
        c: - (line_eq.a * p1.x as f64 + line_eq.b * p1.y as f64),
        p1,
        p2
    }
}

pub fn center_and_flip_lines(width: usize, height: usize, lines: &Vec<LineEquation>) -> Vec<LineEquation> {
    lines.iter()
        .map(|a| center_and_flip(width, height, a))
        .collect::<Vec<LineEquation>>()
}

pub fn flip_back(width: usize, height: usize, point: &Point) -> Point {
    let min_x = -(width as i32) / 2;
    let min_y = -(height as i32) / 2;
    Point::new(point.x - min_x, point.y * -1 - min_y)
}
