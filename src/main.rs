mod video_exporter;
mod text_exporter;
mod opencv_custom;
mod traits;

// Hat description
mod hat;

mod detectors;
mod filters;
mod controllers;
mod point_systems;

// Mainframe
mod main_frame;

use main_frame::MainFrame;

use controllers::mock_controller::MockController;
use detectors::naive_detection::NaiveDetection;
use filters::no_filter::NoFilter;
use point_systems::centralized::Centralized;

use opencv_custom::MyColor;
use crate::hat::Hat;

fn main() {
    let mut s = MainFrame::new(
        NaiveDetection::new(
            Hat::new(
            MyColor::new(0, 20, -20),
            MyColor::new(100, 127, 90),
            5200.0),
            Centralized::new(640, 368)
        ),
        MockController::new("./video-1574588281.mp4"),
        NoFilter::new(Centralized::new(640, 368)),
    );

    s.run();
}
