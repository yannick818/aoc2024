mod day;

use crate::day::*;

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
    measure!("1.1", d1_id_check::cal_distance(&input));
    measure!("1.2", d1_id_check::cal_similarity(&input));

    let input = read_file("input/2.txt");
    measure!("2.1", d2_reports::count_safe(&input));
    measure!("2.2", d2_reports::count_safe_tolerant(&input));

    let input = read_file("input/3.txt");
    measure!("3.1", d3_mull_it_over::multiply_numbers(&input));
    measure!("3.2", d3_mull_it_over::multiply_numbers_filtered(&input));

    let input = read_file("input/4.txt");
    measure!("4.1", d4_ceres_search::count_xmas(&input));
    measure!("4.2", d4_ceres_search::count_x_mas(&input));

    let input = read_file("input/5.txt");
    measure!("5.1", d5_print_queue::count_update(&input));
    measure!("5.2", d5_print_queue::count_corrected(&input));

    let input = read_file("input/6.txt");
    measure!("6.1", d6_guard_gallivant::count_positions(&input));
    measure!("6.2", d6_guard_gallivant::count_possible_block(&input));
}

fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}
