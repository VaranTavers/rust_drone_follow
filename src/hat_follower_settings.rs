/// Settings for HatFollower
pub struct HatFollowerSettings {
    /// Radius of circle around the center that is considered to be okay (if the drone is over this
    /// circle it tries to stay there, otherwise it tries to get over it).
    pub center_threshold: f64,
    /// Minimum amount of change in speeds needed to issue a new move command.
    pub min_change: f64,
    /// Under how many frames(as time measurement) we want the drone to reach the center.
    pub frames_to_be_centered: f64,
    /// Sets whether the program should save the video.
    pub save_to_file: Option<String>,
    /// Sets whether the program should save commands in a file denoting the frame
    pub save_commands: Option<String>,
    /// Sets whether the program should show the image real-time.
    pub show_video: bool,
    /// Sets whether the program should draw the detection markers on the video.
    pub draw_detection: bool,
    /// Sets whether the program should draw the filter markers on the video.
    pub draw_filter: bool,
    /// Sets whether the program should draw a circle marking the center_threshold.
    pub draw_center: bool,
    /// Experimental feature, tries to counteract the speed of the hat. Might not work well.
    pub counteract_velocity: bool,
    /// Turn only when above the target
    pub turn_only_when_centered: bool,

}

impl HatFollowerSettings {
    /// Returns settings with a default setting: (show video, no save, no draw)
    pub fn new() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: None,
            save_commands: None,
            show_video: true,
            draw_detection: false,
            draw_filter: false,
            draw_center: false,
            counteract_velocity: false,
            turn_only_when_centered: true,
        }
    }

    /// Returns settings with full debug setting: (show video, save video, draw all)
    pub fn debug() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: Some(String::from("debug_video.mp4")),
            save_commands: Some(String::from("debug_commands.txt")),
            show_video: true,
            draw_detection: true,
            draw_filter: true,
            draw_center: true,
            counteract_velocity: false,
            turn_only_when_centered: true,
        }
    }

    /// Returns settings with silent setting: (no video, no save, no draw)
    pub fn silent() -> HatFollowerSettings {
        HatFollowerSettings {
            center_threshold: 5.0,
            min_change: 0.3,
            frames_to_be_centered: 10.0,
            save_to_file: None,
            save_commands: None,
            show_video: false,
            draw_detection: false,
            draw_filter: false,
            draw_center: false,
            counteract_velocity: false,
            turn_only_when_centered: true,
        }
    }
}