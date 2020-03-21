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

    /// Should halt all movement
    fn stop(&mut self) {
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

    /// Conversion rate between pixels/dt and drone speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames
    fn get_kv(&self) -> f64 {
        1.0
    }

    /// Conversion rate between da/dt and drone turn speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames, and da is the angle difference between frames.
    fn get_ka(&self) -> f64 {
        1.0
    }
}
