use crate::traits::Controller;

pub struct MockController {
    filename: String
}

impl MockController {
    pub fn new(filename: &str) -> MockController {
        MockController {
            filename: String::from(filename)
        }
    }
}

impl Controller for MockController {
    fn init(&mut self) {
    }
    fn shutdown(&mut self) {
    }

    fn takeoff(&mut self) {
    }
    fn land(&mut self) {
    }

    fn move_all(&mut self, _left_right: f64, _back_front: f64, _down_up: f64, _turn_left_right: f64) {
    }
    fn move_forward(&mut self, _speed: f64) {
    }
    fn move_backward(&mut self, _speed: f64) {
    }
    fn move_left(&mut self, _speed: f64) {
    }
    fn move_right(&mut self, _speed: f64) {
    }
    fn move_up(&mut self, _speed: f64) {
    }
    fn move_down(&mut self, _speed: f64) {
    }

    /// Should halt all movement
    fn stop(&mut self) {
    }

    /// Should return height in cm-s
    fn get_height(&self) -> f64 {
        0.0
    }

    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String {
        self.filename.clone()
    }

    fn get_video_height(&self) -> usize {
        348
    }

    fn get_video_width(&self) -> usize {
        640
    }
}
