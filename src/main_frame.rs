use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;
use opencv::imgproc::{LINE_8, circle};

use crate::video_exporter::VideoExporter;
use crate::traits::*;

pub struct MainFrame<D: Detector, C: Controller> {
    detector: D,
    controller: C,
    point: Point,
    speed: f64,
    center_threshold: f64,
}

impl<D: Detector, C: Controller> MainFrame<D, C>{
    pub fn new(detector: D, controller: C) -> MainFrame<D, C> {
        MainFrame {
            detector,
            controller,
            point: Point::new(0, 0),
            speed: 0.5,
            center_threshold: 5.0,
        }
    }

    fn calculate_new_speed() {

    }


    pub fn run(&mut self) {
        let mut video_exporter = VideoExporter::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        loop {
            match video.read(&mut img) {
                Ok(true) => {
                    self.detector.estimate_new_position(&img, None);
                    match self.detector.get_estimated_position() {
                        Some(p) => {
                            circle(&mut img, Point::new(p.x, p.y), 10,
                            Scalar::new(255.0, 0.0, 0.0, 255.0), 2, LINE_8, 1).unwrap();
                        }
                        _ => {
                            circle(&mut img, Point::new(5, 5), 10,
                            Scalar::new(0.0, 0.0, 255.0, 255.0), 3, LINE_8, 1).unwrap();
                        }
                    }

                    video_exporter.save_frame("test.mp4", &img);

                    imshow("Image", &img).unwrap();
                    cv::highgui::wait_key(3).unwrap();
                }
                _ => {
                    break;
                }
            }
        }

        video_exporter.close(); 
        self.controller.land();
        self.controller.shutdown();
    }
}
