use std::fs;

use crate::models::hat::Hat;
use crate::models::lab_color::LabColor;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/// Reads a file which is in the following format:
///
/// video_file_name
///
/// l1 a1 b1
///
/// l2 a2 b2
///
/// hat_size
///
/// Where l1, l2 are in range 0 - 100, a1, a2, b1, b2 are in range -127 - 127 and are integers,
/// hat_size is a double, and video_file_name is a string containing the path to a video file.
///
/// You can use the results to feed in a MockController or a NaiveDetector
///
/// Any other rows after this will not be read.
pub fn read_file(filename: &str) -> (String, Hat) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let rows: Vec<&str> = contents.split('\n').collect::<Vec<&str>>();
    let video_file = String::from(rows[0]);
    let hat_low = rows[1].split(' ').collect::<Vec<&str>>()
        .iter().map(|s| parse_input!(s.trim(), i8)).collect::<Vec<i8>>();
    let hat_high = rows[2].split(' ').collect::<Vec<&str>>()
        .iter().map(|s| parse_input!(s.trim(), i8)).collect::<Vec<i8>>();
    let hat_size = parse_input!(rows[3], f64);

    (video_file, Hat::new(
        LabColor::new(hat_low[0], hat_low[1], hat_low[2]),
        LabColor::new(hat_high[0], hat_high[1], hat_high[2]),
        hat_size
    ))
}