use crate::traits::Controller;

/// The MockController acts as a false controller that provides a video file to the MainFrame along
/// with it's resolution, and does nothing on commands given to it.
///
/// You can use it to test the tracking system on a prerecorded video.
pub struct MockController {
    filename: String,
    height: usize,
    width: usize,
}

impl MockController {
    /// Usage:
    /// ```
    /// # use crate::controllers::MockController;
    ///
    /// # fn main() {
    ///     let controller = MockController::new("video_file.mp4", 640, 368);
    /// # }
    /// ```
    pub fn new(filename: &str, width: usize, height: usize) -> MockController {
        MockController {
            filename: String::from(filename),
            height,
            width
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

    fn get_video_height(&self) -> usize {
        self.height
    }

    fn get_video_width(&self) -> usize {
        self.width
    }

    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String {
        self.filename.clone()
    }

    /// WIP
    fn get_k(&self) -> f64 {
        0.0
    }
}
