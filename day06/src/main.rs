use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn find_unique_substr_end(data_string: &str, substr_length: usize) -> usize {
    for i in 0..(data_string.len() - substr_length) {
        let substr = &data_string[i..i + substr_length];
        let mut all_different = true;

        'char_scan: for (pos, c1) in substr.chars().enumerate() {
            for c2 in substr.chars().skip(pos + 1) {
                if c1 == c2 {
                    all_different = false;
                    break 'char_scan;
                }
            }
        }

        if all_different {
            return i + substr_length;
        }
    }
    0
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let packet_string = BufReader::new(File::open(&filename).expect("File not found"))
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("Part 1: {}", find_unique_substr_end(&packet_string, 4));
    println!("Part 2: {}", find_unique_substr_end(&packet_string, 14));
}
