mod video_exporter;
mod text_exporter;
mod opencv_custom;
mod traits;

// Detectors
mod naive_detection;

// Controllers
mod mock_controller;

// Point systems
mod centralized;

// Mainframe
mod main_frame;

use main_frame::MainFrame;

use mock_controller::MockController;
use naive_detection::NaiveDetection;
use opencv_custom::MyColor;

fn main() {
    let mut s = MainFrame::new(
        NaiveDetection::new(
            (MyColor { l: 0, a: 20, b: -20},
             MyColor { l: 116, a: 127, b: 90}),
             80.0
        ),
        MockController::new("./video-1574588281.mp4"),
    );

    s.run();
}
