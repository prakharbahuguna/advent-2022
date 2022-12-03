use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let buf_file = BufReader::new(File::open(filename).expect("File not found"));
    let mut part1_score = 0;
    let mut part2_score = 0;

    for line in buf_file.lines() {
        let line_str = line.unwrap();
        let mut line_itr = line_str.chars();
        let p1 = line_itr.next().unwrap() as i32 - 65;
        let p2 = line_itr.nth(1).unwrap() as i32 - 88;

        part1_score += p2 + 1 + 3 * ((p2 - p1).rem_euclid(3) + 1).rem_euclid(3);
        part2_score += p2 * 3 + (p1 + (p2 - 1).rem_euclid(3)).rem_euclid(3) + 1;
    }

    println!("Total score for part 1: {}", part1_score);
    println!("Total score for part 2: {}", part2_score);
}
