use crate::hat::Hat;
use std::fs;
use crate::opencv_custom::MyColor;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
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
        MyColor::new(hat_low[0], hat_low[1], hat_low[2]),
        MyColor::new(hat_high[0], hat_high[1], hat_high[2]),
        hat_size
    ))
}