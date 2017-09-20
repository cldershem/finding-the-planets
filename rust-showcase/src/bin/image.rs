extern crate simple_csv;
extern crate png;

use std::env;
use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::str::FromStr;
use png::HasParameters;
use simple_csv::SimpleCsvReader;

fn main(){
    let f = File::open("../long-cadence.csv").unwrap();
    let buf = BufReader::new(f);
    let mut reader = SimpleCsvReader::new(buf);
    let row = reader.next_row().unwrap().unwrap();
    let mut iter = row.iter();
    iter.next(); // dropping time

    let raw: Vec<f64> = iter
        .map(|s| f64::from_str(s).unwrap())
        .collect();
    let maximum = raw
        .iter()
        .fold(std::f64::MIN, |acc, v| acc.max(*v));
    let data: Vec<u8> = raw
        .iter()
        .map(|s| s/maximum)
        .map(|s| 255.0 * s)
        .map(|s| s as u8)
        .collect();

    println!("{}: {:?}", maximum, data);

    let mut path = env::current_dir().unwrap();
    path.push("trappist-1.0.png");

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 11, 11);
    encoder.set(png::ColorType::Grayscale).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data.as_slice()).unwrap();
}
