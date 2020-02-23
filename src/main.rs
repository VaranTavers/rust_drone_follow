use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;
use opencv::imgproc::{LINE_8, circle};

mod video_handler;
mod opencv_custom;
mod traits;

// Trackers
mod naive_detection;

// Controllers
mod mock_controller;

use traits::{Tracker, Controller};
use mock_controller::MockController;
use naive_detection::NaiveDetection;


struct MainFrame<T: Tracker, C: Controller> {
    tracker: T,
    controller: C
}

impl<T: Tracker, C: Controller> MainFrame<T, C>{
    pub fn new(tracker: T, controller: C) -> MainFrame<T, C> {
        MainFrame {
            tracker,
            controller
        }
    }

    pub fn run(&mut self) {
        let mut video_handler = video_handler::VideoHandler::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        loop {
            match video.read(&mut img) {
                Ok(true) => {
                    self.tracker.estimate_new_position(&img);
                    match self.tracker.get_estimated_position() {
                        Some(p) => {
                            circle(&mut img, Point::new(p.x, p.y), 10,
                            Scalar::new(255.0, 255.0, 255.0, 255.0), 1, LINE_8, 1).unwrap();
                        }
                        _ => {
                            circle(&mut img, Point::new(5, 5), 10,
                            Scalar::new(0.0, 0.0, 255.0, 255.0), 3, LINE_8, 1).unwrap();
                        }
                    }

                    video_handler.save_frame("test.mp4", &img);

                    imshow("Image", &img).unwrap();
                    cv::highgui::wait_key(3).unwrap();
                }
                _ => {
                    break;
                }
            }
        }

        video_handler.close();
    }
}


// Certainty should decrease if more candidates are available

fn main() {
    let mut s = MainFrame::new(
        NaiveDetection::new(),
        MockController::new("./video-1574588281.mp4")
    );

    s.run();
}
