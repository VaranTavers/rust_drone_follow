use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;
use opencv::imgproc::{LINE_8, circle};

pub mod video_exporter;
pub mod text_exporter;

pub mod geometric_point;
pub mod opencv_custom;

pub mod hat_file_reader;
pub mod hat;

pub mod traits;

pub mod point_converter;
pub mod detectors;
pub mod filters;
pub mod controllers;

use crate::video_exporter::VideoExporter;
use crate::text_exporter::TextExporter;
use crate::traits::*;
use crate::geometric_point::GeometricPoint;
use crate::point_converter::PointConverter;
use std::sync::mpsc::Receiver;

/// The heart of the following mechanism. This struct orchestrates the three parts, in order to
/// make the drone follow the object. It's only function is run() which initializes the drone, and
/// starts following the person wearing the hat.
pub struct HatFollower<D: Detector, C: Controller, F: Filter> {
    detector: D,
    controller: C,
    filter: F,
    p_c: PointConverter,
    prev_point: GeometricPoint,
    prev_angle: f64,
    center_threshold: f64,
    last_params: (f64, f64, f64, f64),
    stop_channel: Option<Receiver<i32>>,
}

impl<D: Detector, C: Controller, F: Filter> HatFollower<D, C, F> {

    /// Returns a new HatFollower. Can be initialized with any fitting parameter, depending on your
    /// needs.
    pub fn new(detector: D, controller: C, filter: F, stop_channel: Option<Receiver<i32>>) -> HatFollower<D, C, F> {
        HatFollower {
            p_c: PointConverter::new(controller.get_video_width(), controller.get_video_height()),
            detector,
            controller,
            filter,
            prev_point: GeometricPoint::new(0, 0),
            prev_angle: 0.0,
            center_threshold: 5.0,
            last_params: (0.0, 0.0, 0.0, 0.0),
            stop_channel,
        }
    }

    fn calculate_speed_to_center(&self, x: i32) -> f64 {
        let frames_to_be_centered = 10.0;
        if x.abs() as f64 > self.center_threshold {
            return x as f64 / frames_to_be_centered;
        }
        0.0
    }
    fn get_new_speeds(&mut self) -> (f64, f64) {
        let (x,y) = (self.filter.get_estimated_position().unwrap().x, self.filter.get_estimated_position().unwrap().y);
        let vx_to_center = self.calculate_speed_to_center(x);
        let vy_to_center = self.calculate_speed_to_center(y);

        let kv = self.controller.get_kv();
        (
            ((vx_to_center - self.filter.get_estimated_vx()) * kv).min(1.0).max(-1.0),
            ((vy_to_center - self.filter.get_estimated_vy()) * kv).min(1.0).max(-1.0)
        )
    }

    fn control_the_drone(&mut self) {
        let min_change = 0.3;

        let (new_vx, new_vy) = self.get_new_speeds();
        let ka = self.controller.get_ka();
        let new_turn = ((self.filter.get_estimated_angle() - self.prev_angle) * ka).min(1.0).max(-1.0);

        // Check if a minimum change of speed is reached, in order not to have an overflow of move
        // commands if it's not necessary.
        let (old_vx, old_vy, old_vz, old_turn) = self.last_params;
        if (new_vx - old_vx).abs() + (new_vx - old_vy).abs() + (new_turn - old_turn).abs() > min_change {
            self.controller.move_all(new_vx, new_vy, old_vz, old_turn);
            self.last_params = (new_vx, new_vy, old_vz, new_turn);
        }
    }

    /// Initializes the drone, and makes it follow the person wearing the hat. It can only be stopped
    /// by sending a message through the channel whose receiver was given as a parameter in the constructor.
    pub fn run(&mut self) {
        let mut video_exporter = VideoExporter::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        loop {
            if let Some(receiver) = &mut self.stop_channel {
                if let Ok(_) = receiver.try_recv() {
                    break;
                }
            }
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

        self.controller.land();
        self.controller.shutdown();
    }
}
