use opencv::core::Mat;
use opencv::videoio::{VideoCapture, VideoCaptureTrait, CAP_ANY};

use crate::traits::Controller;

/// The MockController acts as a false controller that provides a video file to the MainFrame along
/// with it's resolution, and does nothing on commands given to it.
///
/// You can use it to test the tracking system on a prerecorded video.
pub struct MockController {
    video: VideoCapture,
    height: usize,
    width: usize,
}

impl MockController {
    /// Usage:
    /// ```
    /// use rust_drone_follow::controllers::mock_controller::MockController;
    /// // ...
    /// # fn main() {
    ///     let controller = MockController::new("video_file.mp4", 640, 368);
    /// # }
    /// ```
    pub fn new(filename: &str, width: usize, height: usize) -> MockController {
        MockController {
            video: VideoCapture::from_file(filename, CAP_ANY).unwrap(),
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


    /// Should return current image from drone
    fn get_next_frame(&mut self, img: &mut Mat) -> opencv::Result<bool> {
        self.video.read(img)
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
