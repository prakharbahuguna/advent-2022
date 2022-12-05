use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let buf_file = BufReader::new(File::open(&filename).expect("File not found"));
    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    // let mut crates_pt1 = vec![
    //     VecDeque::from(['Z', 'N']),
    //     VecDeque::from(['M', 'C', 'D']),
    //     VecDeque::from(['P']),
    // ];
    // let mut crates_pt2 = crates_pt1.clone();
    let mut crates_pt1 = vec![
        VecDeque::from(['W', 'B', 'D', 'N', 'C', 'F', 'J']),
        VecDeque::from(['P', 'Z', 'V', 'Q', 'L', 'S', 'T']),
        VecDeque::from(['P', 'Z', 'B', 'G', 'J', 'T']),
        VecDeque::from(['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C']),
        VecDeque::from(['G', 'V', 'B', 'J', 'S']),
        VecDeque::from(['P', 'S', 'Q']),
        VecDeque::from(['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N']),
        VecDeque::from(['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R']),
        VecDeque::from(['V', 'D', 'T', 'R']),
    ];
    let mut crates_pt2 = crates_pt1.clone();

    for line in buf_file.lines() {
        let line_str = line.unwrap();
        let caps = move_re.captures(&line_str);
        if caps.is_none() {
            continue;
        }
        let instructions = caps.unwrap();
        let move_count = instructions
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let move_from = instructions
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1;
        let move_to = instructions
            .get(3)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1;

        let remove1_idx = crates_pt1[move_from].len() - move_count;
        let removed1 = crates_pt1[move_from]
            .drain(remove1_idx..)
            .rev()
            .collect::<Vec<_>>();
        for elem in removed1 {
            crates_pt1[move_to].push_back(elem);
        }

        let remove2_idx = crates_pt2[move_from].len() - move_count;
        let mut removed2 = crates_pt2[move_from]
            .drain(remove2_idx..)
            .collect::<VecDeque<_>>();
        crates_pt2[move_to].append(&mut removed2);
    }

    print!("Part 1: ");
    for stack in crates_pt1.iter() {
        print!("{}", stack.back().unwrap());
    }
    print!("\nPart 2: ");
    for stack in crates_pt2.iter() {
        print!("{}", stack.back().unwrap());
    }
    println!();
}
