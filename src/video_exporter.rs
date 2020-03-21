use opencv as cv;
use cv::core::*;
use cv::videoio::*;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;

/// Can be used to export frames to a video file. The saving runs on a different thread in order
/// not to block the main thread.
pub struct VideoExporter {
    join_handle: Option<thread::JoinHandle<()>>,
    command_sender: Sender<(String, Option<Mat>)>
}

fn video_exporter_thread(rec: Receiver<(String, Option<Mat>)>) {
    let mut video_writers: HashMap<String, VideoWriter> = HashMap::new();

    loop {
        match rec.recv() {
            Ok((_, None)) => {
                break;
            }
            Ok((name, image_option)) => {
                let image = image_option.unwrap();
                if !video_writers.contains_key(&name) {
                    let mut vw = VideoWriter::new(name.as_str(),
                                              VideoWriter::fourcc('F' as i8, 'M' as i8, 'P' as i8, '4' as i8).unwrap(),
                                              30.0,
                                              image.size().unwrap(),
                                              true).unwrap();
                    vw.write(&image).unwrap();
                    video_writers.insert(name, vw);
                } else {
                    let vw = video_writers.get_mut(&name).unwrap();
                    vw.write(&image).unwrap();
                }

            }
            Err(_) => {
                break;
            }
        }
    }
}

impl VideoExporter {
    /// Creates a new VideoExporter with no managed files.
    pub fn new() -> VideoExporter {
        let (command_sender, receiver) = mpsc::channel();
        let join_handle = Some(thread::spawn(move || {
            video_exporter_thread(receiver);
        }));
        VideoExporter {
            join_handle,
            command_sender
        }
    }

    /// Will start writing a file if it isn't managed, otherwise it will append the frame to it.
    pub fn save_frame(&mut self, video_name: &str, img: &Mat) {
        self.command_sender.send(
            (String::from(video_name), Some(img.clone().unwrap()))
            ).unwrap();
    }
}

impl Drop for VideoExporter {
    fn drop(&mut self) {
        self.command_sender.send((String::new(), None)).unwrap();
        self.join_handle.take().unwrap().join().unwrap();
    }
}
