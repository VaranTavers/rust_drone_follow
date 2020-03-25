use opencv as cv;
use cv::core::*;
use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;

pub trait Detector {
    /// Should return the position of the detected object in the descartes coordinate system.
    fn get_detected_position(&self) -> Option<GeometricPoint>;

    /// Should return the angle that the object turning towards.
    fn get_detected_angle(&self) -> Option<f64>;

    /// Should return the certainty of the detection, mostly the certainty of the angle detection.
    fn get_detection_certainty(&self) -> f64;

    /// Should recalculate it's values based on a new image given to it.
    fn detect_new_position(&mut self, img: &Mat, old_pos: Option<Point>, p_c: &PointConverter);

    /// Should display visually some parts of the detection. (optional)
    fn draw_on_image(&self, img: &mut Mat, p_c: &PointConverter);
}

pub trait Filter {
    /// Updates the estimation based on new information.
    fn update_estimation(&mut self, point: Option<GeometricPoint>, angle: Option<f64>, cert: f64);

    /// Returns the estimated position of the hat.
    fn get_estimated_position(&self) -> Option<GeometricPoint>;

    /// Returns the estimated angle of the hat.
    fn get_estimated_angle(&self) -> f64;

    /// Returns the estimated horizontal speed of the hat.
    fn get_estimated_vx(&self) -> f64;

    /// Returns the estimated vertical speed of the hat.
    fn get_estimated_vy(&self) -> f64;

    /// Returns the certainty of the estimation.
    fn get_estimation_certainty(&self) -> f64;

    /// Returns the certainty of the estimation.
    fn draw_on_image(&self, img: &mut Mat, p_c: &PointConverter);
}

pub trait Controller {
    /// Should handle connecting to the drone.
    fn init(&mut self);
    /// Should handle disconnecting from the drone.
    fn shutdown(&mut self);

    /// Should make the drone take off and assume the correct height.
    fn takeoff(&mut self);
    /// Should make the drone land.
    fn land(&mut self);

    /// Negative values ([-1.0, 0.0)) mean going towards the first direction, positive values
    /// ((0.0, 1.0])) mean going towards the second direction.
    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64);
    /// Should halt all movement
    fn stop(&mut self);

    /// Should return the video's height in pixels
    fn get_video_height(&self) -> usize;

    /// Should return the video's width in pixels
    fn get_video_width(&self) -> usize;

    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String;

    /// Conversion rate between pixels/dt and drone speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames
    fn get_kv(&self) -> f64;

    /// Conversion rate between da/dt and drone turn speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames, and da is the angle difference between frames.
    fn get_ka(&self) -> f64;

}
