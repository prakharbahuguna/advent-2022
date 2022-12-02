use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("Please specify the input filename");
    let buf_file = BufReader::new(File::open(filename).expect("File not found"));
    let mut part1_score = 0u32;
    let mut part2_score = 0u32;

    for line in buf_file.lines() {
        let line_str = line.unwrap();
        let mut line_itr = line_str.chars();
        let p1 = line_itr.next().unwrap();
        let p2 = line_itr.nth(1).unwrap();

        part1_score += match p2 {
            'X' => {
                1 + match p1 {
                    'A' => 3,
                    'C' => 6,
                    _ => 0,
                }
            }
            'Y' => {
                2 + match p1 {
                    'A' => 6,
                    'B' => 3,
                    _ => 0,
                }
            }
            'Z' => {
                3 + match p1 {
                    'B' => 6,
                    'C' => 3,
                    _ => 0,
                }
            }
            _ => 0,
        };

        part2_score += match p2 {
            'X' => match p1 {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            },
            'Y' => {
                3 + match p1 {
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
                    _ => 0,
                }
            }
            'Z' => {
                6 + match p1 {
                    'A' => 2,
                    'B' => 3,
                    'C' => 1,
                    _ => 0,
                }
            }
            _ => 0,
        };
    }

    println!("Total score for part 1: {}", part1_score);
    println!("Total score for part 2: {}", part2_score);
}
