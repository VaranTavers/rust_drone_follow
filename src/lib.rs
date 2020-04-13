pub mod hat_follower_settings;

pub mod video_exporter;
pub mod text_exporter;

pub mod geometric_point;
pub mod opencv_custom;
pub mod marker_drawer;

pub mod hat_file_reader;
pub mod hat;

pub mod traits;

pub mod point_converter;
pub mod detectors;
pub mod filters;
pub mod controllers;

use std::sync::mpsc::Receiver;

use opencv as cv;
use cv::core::*;
use cv::highgui::*;
use cv::videoio::*;

use crate::video_exporter::VideoExporter;
use crate::traits::*;
use crate::point_converter::PointConverter;
use crate::hat_follower_settings::HatFollowerSettings;
use opencv::imgproc::{circle, LINE_8};
use crate::opencv_custom::get_red;
use crate::text_exporter::TextExporter;
use crate::marker_drawer::MarkerDrawer;

/// The heart of the following mechanism. This struct orchestrates the three parts, in order to
/// make the drone follow the object. It's only function is run() which initializes the drone, and
/// starts following the person wearing the hat.
pub struct HatFollower<D: Detector, C: Controller, F: Filter> {
    detector: D,
    controller: C,
    filter: F,
    p_c: PointConverter,
    prev_angle: f64,
    last_params: (f64, f64, f64, f64),
    stop_channel: Option<Receiver<i32>>,
    settings: HatFollowerSettings,
}

impl<D: Detector, C: Controller, F: Filter> HatFollower<D, C, F> {

    /// Returns a new HatFollower. Can be initialized with any fitting parameter, depending on your
    /// needs.
    ///
    /// !!! Beware, if you use a real controller and you don't pass a Receiver when instantiating this or/and you don't run it on a separate
    /// thread it will run indefinitely !!!
    ///
    /// Usage example:
    /// ```
    /// use rust_drone_follow::detectors::naive_detector::NaiveDetector;
    /// use rust_drone_follow::hat::Hat;
    /// use rust_drone_follow::opencv_custom::LabColor;
    /// use rust_drone_follow::controllers::mock_controller::MockController;
    /// use rust_drone_follow::filters::no_filter::NoFilter;
    /// use rust_drone_follow::HatFollower;
    /// use rust_drone_follow::hat_follower_settings::HatFollowerSettings;
    ///
    /// fn main() {
    ///     let mut s = HatFollower::new(
    ///        NaiveDetector::new(Hat::new(
    ///            LabColor::new(0, 20, -127),
    ///            LabColor::new(80, 127, -20),
    ///            1200.0
    ///        )),
    ///        MockController::new("test.mp4", 1280, 720),
    ///        NoFilter::new(),
    ///        HatFollowerSettings::new(),
    ///        None,
    ///    );
    /// }
    /// ```
    pub fn new(detector: D, controller: C, filter: F, settings: HatFollowerSettings, stop_channel: Option<Receiver<i32>>) -> HatFollower<D, C, F> {
        HatFollower {
            p_c: PointConverter::new(controller.get_video_width(), controller.get_video_height()),
            detector,
            controller,
            filter,
            prev_angle: 0.0,
            last_params: (0.0, 0.0, 0.0, 0.0),
            stop_channel,
            settings
        }
    }

    // Calculates the necessary speed that is needed for the hat to be in the center of the frame.
    // It is in dx/dt where dx is the coordinate difference and dt is the time between frames.
    fn calculate_speed_to_center(&self, dist: i32) -> f64 {
        if dist.abs() as f64 > self.settings.center_threshold {
            return dist as f64 / self.settings.frames_to_be_centered;
        }
        0.0
    }

    fn calculate_new_vs(&mut self) -> (f64, f64) {
        // If there is no detected point, the drone should stop.
        if let None = self.filter.get_estimated_position() {
            return (0.0, 0.0);
        }

        let est_position = self.filter.get_estimated_position().unwrap();
        let mut vx_to_center = self.calculate_speed_to_center(est_position.x);
        let mut vy_to_center = self.calculate_speed_to_center(est_position.y);

        // Feature that needs testing.
        if self.settings.counteract_velocity {
            vx_to_center -= self.filter.get_estimated_vx();
            vy_to_center -= self.filter.get_estimated_vy();
        }

        let kv = self.controller.get_kv();
        (
            ((vx_to_center) * kv).min(1.0).max(-1.0),
            ((vy_to_center) * kv).min(1.0).max(-1.0)
        )
    }

    fn control_the_drone(&mut self, frame_num: usize, text_exporter: &mut TextExporter) {
        let min_change = self.settings.min_change;

        let (new_vx, new_vy) = self.calculate_new_vs();
        let ka = self.controller.get_ka();
        let new_turn = ((self.filter.get_estimated_angle() - self.prev_angle) * ka).min(1.0).max(-1.0);

        // Check if a minimum change of speed is reached, in order not to have an overflow of move
        // commands if it's not necessary.
        let (old_vx, old_vy, old_vz, old_turn) = self.last_params;
        if (new_vx - old_vx).abs() + (new_vy - old_vy).abs() + (new_turn - old_turn).abs() > min_change {
            if let Some(filename) = &self.settings.save_commands {
                text_exporter.save_row(filename.as_str(), format!("{} {} {} {} {}\n", frame_num, new_vx, new_vy, old_vz, new_turn));
            }
            self.controller.move_all(new_vx, new_vy, old_vz, new_turn);
            self.last_params = (new_vx, new_vy, old_vz, new_turn);
        }
    }

    fn main_loop(&mut self, img: &mut Mat, frame_num: usize, video_exporter: &mut VideoExporter, text_exporter: &mut TextExporter) {
        let mut m_d = MarkerDrawer::new();
        let point_for_detector = self.filter.get_estimated_position();
        self.detector.detect_new_position(
            &img,
            point_for_detector.map(|gp| self.p_c.convert_to_image_coords( &gp)),
            &self.p_c);

        self.filter.update_estimation(
            self.detector.get_detected_position(),
            self.detector.get_detected_angle(),
            self.detector.get_detection_certainty()
        );

        // Drawing on the image
        if self.settings.draw_detection {
            self.detector.draw_on_image(&mut m_d);
        }
        if self.settings.draw_filter {
            self.filter.draw_on_image(&mut m_d);
        }
        if self.settings.draw_center {
            m_d.circle(&self.p_c.get_center(), self.settings.center_threshold as i32, get_red());
        }

        // Applies the given markers to the image.
        m_d.draw_on_image(img, &self.p_c);

        // Save to video file
        if let Some(filename) = &self.settings.save_to_file {
            video_exporter.save_frame(filename.as_str(), img);
        }

        self.control_the_drone(frame_num, text_exporter);

        // Show video file
        if self.settings.show_video {
            imshow("Image", img).unwrap();
            cv::highgui::wait_key(3).unwrap();
        }
    }

    fn have_received_stop_command(&mut self) -> bool {
        if let Some(receiver) = &mut self.stop_channel {
            if let Ok(_) = receiver.try_recv() {
                return true;
            }
        }
        false
    }

    /// Initializes the drone, and makes it follow the person wearing the hat. It can only be stopped
    /// by sending a message through the channel whose receiver was given as a parameter in the constructor.
    pub fn run(&mut self) {
        self.controller.init();
        self.controller.takeoff();

        let mut video_exporter = VideoExporter::new();
        let mut text_exporter = TextExporter::new();
        let mut video = VideoCapture::new_from_file_with_backend(self.controller.get_opencv_url().as_str(), CAP_ANY).unwrap();
        let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
        let mut frame_num = 1;
        loop {
            if self.have_received_stop_command() {
                break;
            }
            match video.read(&mut img) {
                Ok(true) => {
                    self.main_loop(&mut img, frame_num, &mut video_exporter, &mut text_exporter);
                }
                _ => {
                    break;
                }
            }
            frame_num += 1;
        }

        self.controller.land();
        self.controller.shutdown();
    }
}
