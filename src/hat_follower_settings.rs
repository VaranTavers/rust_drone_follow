/// Settings for HatFollower
pub struct HatFollowerSettings {
    /// Radius of circle around the center that is considered to be okay (if the drone is over this
    /// circle it tries to stay there, otherwise it tries to get over it).
    pub center_threshold: f64,
    /// Minimum amount of change in speeds needed to issue a new move command.
    pub min_change: f64,
    /// Under how many frames(as time measurement) we want the drone to reach the center
    pub frames_to_be_centered: f64,
    /// Should the program save the video
    pub save_to_file: Option<String>,
    /// Should the program show the image real-time
    pub show_video: bool,
    /// Should the program draw the detection markers on the video
    pub draw_detection: bool,
    /// Should the program draw the filter markers on the video
    pub draw_filter: bool,

}

impl HatFollowerSettings {
    /// Returns settings with a default setting: (show video, no save, no draw)
    pub fn new() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: None,
            show_video: true,
            draw_detection: false,
            draw_filter: false,
        }
    }

    /// Returns settings with full debug setting: (show video, save video, draw all)
    pub fn debug() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: Some(String::from("test.mp4")),
            show_video: true,
            draw_detection: true,
            draw_filter: true,
        }
    }

    /// Returns settings with silent setting: (no video, no save, no draw)
    pub fn silent() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: None,
            show_video: false,
            draw_detection: false,
            draw_filter: false,
        }
    }
}