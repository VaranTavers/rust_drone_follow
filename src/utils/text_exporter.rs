use std::fs::{File};
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;

pub struct TextExporter {
    join_handle: Option<thread::JoinHandle<()>>,
    command_sender: Sender<(String, Option<String>)>
}

/// Can be used to save text in the same way VideoExporter does. The saving runs on a different
/// thread in order not to block the main thread.
fn text_exporter_thread(rec: Receiver<(String, Option<String>)>) {
    let mut text_writers: HashMap<String, File> = HashMap::new();

    loop {
        match rec.recv() {
            Ok((_, None)) => {
                break;
            }
            Ok((name, text_option)) => {
                let text = text_option.unwrap();
                if !text_writers.contains_key(&name) {
                    let mut file = File::create(&name).unwrap();
                    file.write(text.as_bytes()).unwrap();
                    text_writers.insert(name, file);
                } else {
                    let file = text_writers.get_mut(&name).unwrap();
                    file.write(text.as_bytes()).unwrap();
                }

            }
            Err(_) => {
                break;
            }
        }
    }
}

impl TextExporter {

    /// Creates a new TextExporter with no managed files.
    pub fn new() -> TextExporter {
        let (command_sender, receiver) = mpsc::channel();
        let join_handle = Some(thread::spawn(move || {
            text_exporter_thread(receiver);
        }));
        TextExporter {
            join_handle,
            command_sender
        }
    }

    /// Will start writing a file if it isn't managed, otherwise it will append the row to it.
    pub fn save_row(&mut self, text_name: &str, text: String) {
        self.command_sender.send(
            (String::from(text_name), Some(text))
            ).unwrap();
    }
}

impl Drop for TextExporter {
    fn drop(&mut self) {
        self.command_sender.send((String::new(), None)).unwrap();
        self.join_handle.take().unwrap().join().unwrap();
    }
}
