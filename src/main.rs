mod video_exporter;
mod text_exporter;
mod geometric_point;
mod opencv_custom;
mod traits;

// Hat description
mod hat_file_reader;
mod hat;
mod point_converter;
mod detectors;
mod filters;
mod controllers;

// Mainframe
mod main_frame;

use main_frame::MainFrame;

use controllers::mock_controller::MockController;
use detectors::naive_detector::NaiveDetector;
use filters::no_filter::NoFilter;

use opencv_custom::LabColor;
use crate::hat::Hat;

fn main() {
    let (filename, hat) = hat_file_reader::read_file("kek.hat");
    let mut s = MainFrame::new(
        NaiveDetector::new(hat),
        MockController::new(filename.as_str(), 1280, 720),
        NoFilter::new(),
    );

    s.run();
}
