use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let buf_file = BufReader::new(File::open(&filename).expect("File not found"));
    let mut fully_contained_count = 0u32;
    let mut overlap_count = 0u32;

    // Part 1
    for line in buf_file.lines() {
        let result: Vec<_> = line
            .unwrap()
            .split(&[',', '-'])
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();
        if (result[0] <= result[2] && result[1] >= result[3])
            || (result[2] <= result[0] && result[3] >= result[1])
        {
            fully_contained_count += 1;
        }
        if (result[0] <= result[2] && result[2] <= result[1])
            || (result[2] <= result[0] && result[0] <= result[3])
        {
            overlap_count += 1
        }
    }

    println!(
        "Number of fully contained ranges: {}",
        fully_contained_count
    );
    println!("Number of overlapping ranges: {}", overlap_count);
}
