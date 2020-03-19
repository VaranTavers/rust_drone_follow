/// A new type for points that ensures type-safety: No point in the image's coordinate system gets into
/// calculations and no point from calculations gets used as an input to an OpenCV drawing function.
pub struct GeometricPoint {
    pub x: i32,
    pub y: i32
}

impl Clone for GeometricPoint {
    fn clone(&self) -> Self {
        GeometricPoint {
            x: self.x,
            y: self.y,
        }
    }
}

impl GeometricPoint {
    pub fn new(x: i32, y: i32) -> GeometricPoint {
        GeometricPoint { x, y }
    }
    pub fn d(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
pub fn get_center_of_geometric_points(contour: &Vec<GeometricPoint>) -> GeometricPoint {
    let (s_x, s_y) = contour
        .iter()
        .fold((0, 0), |(a_x, a_y), p| (a_x + p.x, a_y + p.y));
    let l = contour.len();
    GeometricPoint::new(s_x / (l as i32), s_y / (l as i32))
}

pub fn get_closest_from_geometric_points_to_point(contour: &Vec<GeometricPoint>, c: &GeometricPoint) -> (GeometricPoint, i32) {
    contour.iter()
        .fold((GeometricPoint::new(0, 0), 5000000), |(p, d), c_p| {
            let dist_sq = (c.x - c_p.x).pow(2) + (c.y - c_p.y).pow(2);
            if dist_sq < d {
                return (c_p.clone(), dist_sq);
            }
            (p, d)
        })
}
