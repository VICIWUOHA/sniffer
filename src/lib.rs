use std::io::{BufRead, BufReader};

pub fn read_number_lines_in_file(file_path: &str) -> u32 {
    let mut count = 0;
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    for _ in bf.lines() {
        count += 1;
    }
    count
}

pub fn print_headers(file_path: &str, delimiter: &str) {
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    let mut count = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        let headers = line.split(delimiter).collect::<Vec<&str>>();
        println!("Headers: {:?}", headers);
        break;
    }
}

pub fn print_a_few_lines(file_path: &str, delimiter: &str, number_of_lines: u32) {
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    let mut count = 0;
    for line in bf.lines() {
        if count == 0 {
            // skip the first line
            count += 1;
            continue;
        }
        let line = line.unwrap();
        println!("Row: {:?}", line);
        count += 1;
        if count == number_of_lines {
            break;
        }
    }
}