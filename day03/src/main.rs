use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let letter_to_priority = |c: char| {
        if c.is_uppercase() {
            c as u32 - 38
        } else {
            c as u32 - 96
        }
    };
    let mut priority_sum = 0u32;

    // Part 1
    let mut buf_file = BufReader::new(File::open(&filename).expect("File not found"));
    for line in buf_file.lines() {
        let line_str = line.unwrap();
        let (left_side, right_side) = line_str.split_at(line_str.len() / 2);
        for c in left_side.chars() {
            if right_side.contains(c) {
                priority_sum += letter_to_priority(c);
                break;
            }
        }
    }

    println!("Part 1 sum of priorities: {}", priority_sum);

    // Part 2
    buf_file = BufReader::new(File::open(&filename).expect("File not found"));
    let mut file_lines = buf_file.lines();
    priority_sum = 0;

    // I'm not sure of a more elegant way of iterating over the buffer N lines at a time
    // Please let me know if you know a more idiomatic way of doing this.
    while let (Some(line1), Some(line2), Some(line3)) =
        (file_lines.next(), file_lines.next(), file_lines.next())
    {
        let mut chars1: HashSet<char> = HashSet::from_iter(line1.unwrap().chars());
        let chars2: HashSet<char> = HashSet::from_iter(line2.unwrap().chars());
        let chars3: HashSet<char> = HashSet::from_iter(line3.unwrap().chars());

        // I would use intersection() here but that yields an iterator which you can't
        // easily chain with another intersection() with the third set. intersection()
        // would have been fine with just two sets.
        chars1.retain(|c| chars2.contains(c) && chars3.contains(c));
        priority_sum += letter_to_priority(*chars1.iter().next().unwrap());
    }

    println!("Part 2 sum of priorities: {}", priority_sum);
}
