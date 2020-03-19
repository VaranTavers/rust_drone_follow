mod video_exporter;
mod text_exporter;
mod geometric_point;
mod opencv_custom;
mod traits;

// Hat description
mod hat;
mod point_converter;
mod detectors;
mod filters;
mod controllers;

// Mainframe
mod main_frame;

use main_frame::MainFrame;

use controllers::mock_controller::MockController;
use detectors::naive_detection::NaiveDetection;
use filters::no_filter::NoFilter;

use opencv_custom::MyColor;
use crate::hat::Hat;

fn main() {
    let mut s = MainFrame::new(
        NaiveDetection::new(
            Hat::new(
            MyColor::new(0, 20, -20),
            MyColor::new(100, 127, 90),
            5200.0)),
        MockController::new("./video-1574588281.mp4"),
        NoFilter::new(),
    );

    s.run();
}
