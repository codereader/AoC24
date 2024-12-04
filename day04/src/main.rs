use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines: Vec<_> = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect();

    let mut sum_part1 = 0;

    for line in lines.clone() {
        sum_part1 += line.match_indices("XMAS").count();
        sum_part1 += line.match_indices("SAMX").count();
    }

    let height = lines.len();

    for y in 0..height-3 {
        let width = lines[y].len();
        for x in 0..width {
            // Diagonal down
            if x < width - 3 {
                // diagonal forward
                if lines[y].chars().nth(x).unwrap() == 'X' &&
                   lines[y + 1].chars().nth(x + 1).unwrap() == 'M' &&
                   lines[y + 2].chars().nth(x + 2).unwrap() == 'A' && 
                   lines[y + 3].chars().nth(x + 3).unwrap() == 'S' {
                    sum_part1 += 1;
                }

                // diagonal backwards
                if lines[y].chars().nth(x).unwrap() == 'S' &&
                   lines[y + 1].chars().nth(x + 1).unwrap() == 'A' &&
                   lines[y + 2].chars().nth(x + 2).unwrap() == 'M' && 
                   lines[y + 3].chars().nth(x + 3).unwrap() == 'X' {
                    sum_part1 += 1;
                }

                // Diagonal up forward
                if lines[y].chars().nth(x + 3).unwrap() == 'S' &&
                   lines[y + 1].chars().nth(x + 2).unwrap() == 'A' &&
                   lines[y + 2].chars().nth(x + 1).unwrap() == 'M' && 
                   lines[y + 3].chars().nth(x + 0).unwrap() == 'X' {
                    sum_part1 += 1;
                }

                // diagonal up backward
                if lines[y].chars().nth(x + 3).unwrap() == 'X' &&
                   lines[y + 1].chars().nth(x + 2).unwrap() == 'M' &&
                   lines[y + 2].chars().nth(x + 1).unwrap() == 'A' && 
                   lines[y + 3].chars().nth(x + 0).unwrap() == 'S' {
                    sum_part1 += 1;
                }
            }

            // Vertical down
            if lines[y].chars().nth(x).unwrap() == 'X' &&
                lines[y + 1].chars().nth(x).unwrap() == 'M' &&
                lines[y + 2].chars().nth(x).unwrap() == 'A' && 
                lines[y + 3].chars().nth(x).unwrap() == 'S' {
                sum_part1 += 1;
            }

            // Vertical up
            if lines[y].chars().nth(x).unwrap() == 'S' &&
                lines[y + 1].chars().nth(x).unwrap() == 'A' &&
                lines[y + 2].chars().nth(x).unwrap() == 'M' && 
                lines[y + 3].chars().nth(x).unwrap() == 'X' {
                sum_part1 += 1;
            }
        }
    }

    println!("[Part1]: XMAS occurrences: {0}", sum_part1); // 2536

    println!("[Part2]: TODO");
}
