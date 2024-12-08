mod day;

use crate::day::*;

#[allow(unused_imports)]
use std::time::Instant;
use std::{fs::File, io::Read};

macro_rules! measure {
    ($title:expr, $func:expr) => {{
        let start = Instant::now();
        let result = $func;
        let duration = start.elapsed();
        let time = if duration.as_secs() / 60 > 1 {
            format!("{} min", duration.as_secs())
        } else if duration.as_secs() > 1 {
            format!("{} s  ", duration.as_secs())
        } else if duration.as_millis() > 1 {
            format!("{} ms ", duration.as_millis())
        } else if duration.as_micros() > 1 {
            format!("{} us ", duration.as_micros())
        } else {
            format!("{} ns ", duration.as_nanos())
        };
        println!("Day {:>4} in {:>8}: {}", $title, time, result);
    }};
}

fn main() {

    let input = read_file("input/1.txt");
    // measure!("1.1", d1_trebuchet::cal_trebuchet(&input)?);
    // measure!("1.2", d1_trebuchet::cal_trebuchet_str(&input)?);

}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}
