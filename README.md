# rust_hat_follow

This is a Rust library that aims to be able to control a drone, in order to have it follow a hat, that it detects with
it's downwards facing camera. (a compatible drone is for example: Parrot AR Drone 2.0)

This library contains some basic detectors and filters but is extendable by the traits they implement.

## HatFollower
The heart of this library is the HatFollower struct that is generic with three types. It needs a Detector, a Controller, 
and a Filter.

```rust
pub fn new(detector: D, controller: C, filter: F) -> HatFollower<D, C, F>;
```

It has a `run` function that will start up the drone and start following the hat.

Example of instantiation, and usage:

```rust
fn main() {                                                                       
    let mut s = HatFollower::new(
        NaiveDetector::new(Hat::new(
                                   LabColor::new(0, 20, -127),
                                   LabColor::new(80, 127, -20),
                                   1200.0
                               )),
        MockController::new("test.mp4", 1280, 720),
        NoFilter::new(),
    );
    s.run();                                                                      
}
```

## Detector

A detector is the part of the system that processes the video-frames, detects the hat on it (if it is there) and saves
it's coordinates, transforming it to an descartes coordinate system, which has it's central point in the middle of the
video frame. It also calculates the angle that the hat is facing. (in general it works with baseball caps)

```rust
pub trait Detector {
    /// Should return the position of the detected object in the descartes coordinate system.
    fn get_detected_position(&self) -> Option<GeometricPoint>;

    /// Should return the angle that the object turning towards.
    fn get_detected_angle(&self) -> Option<f64>;

    /// Should return the certainty of the detection, mostly the certainty of the angle detection.
    fn get_detection_certainty(&self) -> f64;

    /// Should recalculate it's values based on a new image given to it.
    fn detect_new_position(&mut self, img: &Mat, old_pos: Option<Point>, p_c: &PointConverter);

    /// Should display visually some parts of the detection. (optional)
    fn draw_on_image(&self, img: &mut Mat, p_c: &PointConverter);
}
```

### NaiveDetector

This detector is included with the library. It requires a Hat struct which encodes three properties:

 - The color with lowest accepted values in Lab color space
 
 - The color with highest accepted values in Lab color space
 
 - The average size of the hat.
 
 Example (looks for a dark blue baseball cap):
 
 ```rust
    let hat = Hat::new(
        LabColor::new(0, 20, -127),
        LabColor::new(80, 127, -20),
        1200.0
    );
```

If we have this struct we can simply instantiate our NaiveDetector:

```rust
    let naive_detector = NaiveDetector::new(hat);
```

We can detect the hat on a new image by calling it's `detect_new_position` method.

This detector will always ignore previous positions, and will only use information from the new video-frame.

## Controller

The controller is the part of the system that handles communication between the drone and the HatFollower. It also
provides important information about certain properties of the drone, such as video resolution, and speed multiplier
(meaning how fast should the drone move to travel a given distance calculated from the frame).

```rust
pub trait Controller {
    /// Should handle connecting to the drone.
    fn init(&mut self);
    /// Should handle disconnecting from the drone.
    fn shutdown(&mut self);

    /// Should make the drone take off and assume the correct height.
    fn takeoff(&mut self);
    /// Should make the drone land.
    fn land(&mut self);

    /// Negative values ([-1.0, 0.0)) mean going towards the first direction, positive values
    /// ((0.0, 1.0])) mean going towards the second direction.
    fn move_all(&mut self, left_right: f64, back_front: f64, down_up: f64, turn_left_right: f64);
    fn move_forward(&mut self, speed: f64);
    fn move_backward(&mut self, speed: f64);
    fn move_left(&mut self, speed: f64);
    fn move_right(&mut self, speed: f64);
    fn move_up(&mut self, speed: f64);
    fn move_down(&mut self, speed: f64);

    /// Should halt all movement
    fn stop(&mut self);

    /// Should return height in cm-s
    fn get_height(&self) -> f64;

    /// Should return the video's height in pixels
    fn get_video_height(&self) -> usize;
    /// Should return the video's width in pixels
    fn get_video_width(&self) -> usize;
    /// Should return a link to an external resource that OpenCV can read
    fn get_opencv_url(&self) -> String;
    /// Conversion rate between pixels/dt and drone speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames
    fn get_k(&self) -> f64;
    /// Conversion rate between da/dt and drone turn speed which is in (-1.0, 1.0), where dt is the
    /// time difference between frames, and da is the angle difference between frames.
    fn get_ka(&self) -> f64;
}
```

### MockController

This library only provides a MockController, that returns a link to a video file which the HatFollower will read, and 
it ignores all commands given to it. It is useful to test the detection on prerecorded videos.

The video will be read by OpenCV, so any format supported by it will be supported by the MockController too.

To instantiate you have to give it a path string to the video file, and the resolution of the video.

```rust
let mock_controller = MockController::new("test.mp4", 1280, 720);
```

## Filter

The filter is the part of the system that is responsible for making sure, that no errors during detection mess up the
tracking and following of the hat. It is also responsible for calculating the relative speed of the hat compared to the
drone.

```rust
pub trait Filter {
    /// Updates the estimation based on new information.
    fn update_estimation(&mut self, point: &GeometricPoint, angle: f64, cert: f64);

    /// Returns the estimated position of the hat.
    fn get_estimated_position(&self) -> Option<GeometricPoint>;
    /// Returns the estimated angle of the hat.
    fn get_estimated_angle(&self) -> f64;
    /// Returns the estimated horizontal speed of the hat.
    fn get_estimated_vx(&self) -> f64;
    /// Returns the estimated vertical speed of the hat.
    fn get_estimated_vy(&self) -> f64;
    /// Returns the certainty of the estimation.
    fn get_estimation_certainty(&self) -> f64;

    /// Returns the certainty of the estimation.
    fn draw_on_image(&self, img: &mut Mat, p_c: &PointConverter);
}
```

### NoFilter

This library includes a filter that does no filtering, and only calculates the speed from the difference of the last 
point and the current point.

Example:
```rust
let no_filter = NoFilter::new();
```

### KalmanFilter

This part is not yet implemented. Check back later.

## Other useful utilities:

### VideoExporter

Can be used to export frames to multiple video files.

Usage:
```rust
fn main() {
    let mut video_exporter = VideoExporter::new();
    let mut video = VideoCapture::new_from_file_with_backend("video_file.mp4", CAP_ANY).unwrap();
    let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
    loop {
        match video.read(&mut img) {
            Ok(true) => {
                // Draw something to the frame
                // ...
                video_exporter.save_frame("test.mp4", &img);
                }
                _ => {
                    break;
                }
            }
        }
}
```

### TextExporter

Can be used to save text to multiple files simultaneously.

Usage:
```rust
fn main() {
    let mut text_exporter = TextExporter::new();
    let mut video = VideoCapture::new_from_file_with_backend("video_file.mp4", CAP_ANY).unwrap();
    let mut img = Mat::zeros_size(Size::new(1,1), CV_8U).unwrap().to_mat().unwrap();
    loop {
        match video.read(&mut img) {
            Ok(true) => {
                let data = 42;
                // Do some calculations
                // ...
                text_exporter.save_frame("test.mp4", format!("{}", data));
                }
                _ => {
                    break;
                }
            }
        }
}
```

### HatFileReader

Reads a file which is in the following format:

```text
video_file_name
l1 a1 b1
l2 a2 b2
hat_size
```
Where l1, l2 are in range 0 - 100, a1, a2, b1, b2 are in range -127 - 127 and are integers, 
hat_size is a double, and video_file_name is a string containing the path to a video file.

You can use the results to feed in a MockController or a NaiveDetector

Any other rows after this will not be read.

Usage: 

```rust
    let (filename, hat) = hat_file_reader::read_file("kek.hat");
```

Example file:
```text
./kek.mp4                                                                         
0 20 -127                                                                         
80 127 -20                                                                        
15200.0
```

### PointConverter

Converts points from OpenCV to points in a descartes coordinate system which has O in the middle of the picture.

Instantiating:
```rust
    let p_c = PointConverter::new(640, 368);
```

After it you can use `convert_from_image_coords` to convert from OpenCV Point to GeometricPoint (used in calculations), 
and you can use `convert_to_image_coords` to convert a GeometricPoint into OpenCV point.