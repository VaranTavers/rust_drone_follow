use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;
use opencv::imgproc::{LINE_8, circle};

use crate::video_exporter::VideoExporter;
use crate::text_exporter::TextExporter;
use crate::traits::*;
use crate::geometric_point::GeometricPoint;
use crate::point_systems::centralized::Centralized;

pub struct MainFrame<D: Detector, C: Controller, F: Filter> {
    detector: D,
    controller: C,
    filter: F,
    speed: f64,
    cent: Centralized,
    prev_point: GeometricPoint,
    center_threshold: f64,
}

impl<D: Detector, C: Controller, F: Filter> MainFrame<D, C, F> {
    pub fn new(detector: D, controller: C, filter: F) -> MainFrame<D, C, F> {
        MainFrame {
            detector,
            controller,
            filter,
            speed: 0.5,
            cent: Centralized::new(controller.get_video_width(), controller.get_video_height());
            prev_point: GeometricPoint::new(0, 0),
            center_threshold: 5.0,
        }
    }

    fn calculate_new_speed(&mut self, new_point: &GeometricPoint) {
        let k = 0.1;
        let center = self.cent.get_center();
        let prev_diff = GeometricPoint::new(self.prev_point.x - center.x, self.prev_point.y - center.y).d();
        let current_diff = GeometricPoint::new(new_point.x - center.x, new_point.y - center.y).d();

        self.speed += (current_diff - prev_diff) * k;
    }

    pub fn run(&mut self) {
        let mut video_exporter = VideoExporter::new();
        let mut text_exporter = TextExporter::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        loop {
            match video.read(&mut img) {
                Ok(true) => {
                    let point_for_detector = self.filter.get_estimated_position_for_detector();
                    self.detector.detect_new_position(&img, point_for_detector);
                    self.detector.draw_on_image(&mut img);
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
        text_exporter.close(); 
        self.controller.land();
        self.controller.shutdown();
    }
}
