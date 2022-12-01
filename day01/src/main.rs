use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let buf_file = BufReader::new(File::open(filename).expect("File not found"));

    let mut elf_calories: Vec<u32> = vec![];
    let mut current_calories = 0;

    for line in buf_file.lines() {
        let line_str = line.unwrap();
        if line_str.is_empty() {
            elf_calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line_str.parse::<u32>().unwrap_or_default();
        }
    }
    elf_calories.push(current_calories);

    elf_calories.sort_by(|a, b| b.cmp(a));
    let top_n = 3;
    println!("Elf with highest calories: {}", elf_calories[0]);
    println!(
        "Total calories of top {} elves: {}",
        top_n,
        elf_calories[0..top_n].iter().fold(0, |acc, x| acc + x)
    );
}
