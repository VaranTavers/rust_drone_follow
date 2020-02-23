use opencv as cv;
use cv::core::*;

pub trait Tracker {
    fn get_estimated_position(&self) -> Option<Point>;

    fn get_estimated_certainty(&self) -> f64;

    fn estimate_new_position(&mut self, img: &Mat);
}

pub trait Controller {
    fn init(&mut self);
    fn shutdown(&mut self);

    fn takeoff(&mut self);
    fn land(&mut self);

    fn move_forward(&mut self, speed: f64);
    fn move_backward(&mut self, speed: f64);
    fn move_left(&mut self, speed: f64);
    fn move_right(&mut self, speed: f64);
    fn move_up(&mut self, speed: f64);
    fn move_down(&mut self, speed: f64);

    /// Should halt all movement
    fn stop(&mut self);

    /// Should return height in cm-s
    fn get_height(&self) -> f64;

    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String;

}
