use std::fs::{File};
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::collections::HashMap;

pub struct TextExporter {
    join_handle: thread::JoinHandle<()>,
    command_sender: Sender<(String, Option<String>)>
}

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
    pub fn new() -> TextExporter {
        let (command_sender, receiver) = mpsc::channel();
        let join_handle = thread::spawn(move || {
            text_exporter_thread(receiver);
        });
        TextExporter {
            join_handle,
            command_sender
        }
    }

    pub fn save_frame(&mut self, text_name: &str, text: String) {
        self.command_sender.send(
            (String::from(text_name), Some(text))
            ).unwrap();
    }

    pub fn close(self) {
        self.command_sender.send((String::new(), None)).unwrap();
        self.join_handle.join().unwrap();
    }
}
