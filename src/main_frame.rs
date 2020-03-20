use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;
use opencv::imgproc::{LINE_8, circle};

use crate::video_exporter::VideoExporter;
use crate::text_exporter::TextExporter;
use crate::traits::*;
use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;

pub struct MainFrame<D: Detector, C: Controller, F: Filter> {
    detector: D,
    controller: C,
    filter: F,
    p_c: PointConverter,
    prev_point: GeometricPoint,
    center_threshold: f64,
    last_params: (f64, f64, f64, f64)
}

impl<D: Detector, C: Controller, F: Filter> MainFrame<D, C, F> {
    pub fn new(detector: D, controller: C, filter: F) -> MainFrame<D, C, F> {
        MainFrame {
            p_c: PointConverter::new(controller.get_video_width(), controller.get_video_height()),
            detector,
            controller,
            filter,
            prev_point: GeometricPoint::new(0, 0),
            center_threshold: 5.0,
            last_params: (0.0, 0.0, 0.0, 0.0),
        }
    }

    fn control_the_drone(&mut self) {
        let k = 0.1;
        let min_change = 0.1;
        let new_vx = (- self.filter.get_estimated_vx() * k).min(1.0).max(-1.0);
        let new_vy = (- self.filter.get_estimated_vy() * k).min(1.0).max(-1.0);

        // TODO: It's not enough to move in the same speed as the person, we have to keep them in
        // the center. + k has to be tested, might be moved into the Controller.

        // TODO: Calculate turns.

        // Check if a minimum change of speed is reached, in order not to have an overflow of move
        // commands if it's not necessary.
        let (old_vx, old_vy, old_vz, old_turn) = self.last_params;
        if (new_vx - old_vx).abs() + (new_vx - old_vy).abs() > min_change {
            self.controller.move_all(new_vx, new_vy, old_vz, old_turn);
            self.last_params = (new_vx, new_vy, old_vz, old_turn);
        }
    }

    pub fn run(&mut self) {
        let mut video_exporter = VideoExporter::new();
        let mut text_exporter = TextExporter::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        loop {
            match video.read(&mut img) {
                Ok(true) => {
                    let point_for_detector = self.filter.get_estimated_position();
                    self.detector.detect_new_position(
                        &img,
                        point_for_detector.map(|gp| self.p_c.convert_to_image_coords( &gp)),
                    &self.p_c);
                    self.detector.draw_on_image(&mut img, &self.p_c);
                    video_exporter.save_frame("test.mp4", &img);

                    self.control_the_drone();

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
