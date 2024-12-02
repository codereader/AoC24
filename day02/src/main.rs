use std::io::Read;
use std::fs::File;

fn main() {
    
    // Create an empty mutable string
    let mut test_content = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n').filter(|&x| !x.is_empty());

    let mut safe_report_count_part1 = 0;

    for line in lines {
        let pieces = line.split(' ').filter(|&x| !x.is_empty());
        let is_safe = evaluate_report_part1(pieces);
        
        if is_safe {
            safe_report_count_part1 += 1;
        }
    }

    println!("[Part1]: Safe report count = {}", safe_report_count_part1);

    let mut safe_report_count_part2 = 0;
    let lines2 = normalized_file.split('\n').filter(|&x| !x.is_empty());

    for line in lines2 {
        let is_safe = evaluate_report_part2(line);
        
        println!("{0} is {1}", line, is_safe);

        if is_safe {
            safe_report_count_part2 += 1;
        }
    }

    println!("[Part2]: Safe report count = {}", safe_report_count_part2);
}

fn evaluate_report_part1(pieces: impl IntoIterator<Item = impl AsRef<str>>) -> bool {
    let mut should_increase: Option<bool> = None;
    let mut is_safe = true;
    let mut previous_value: Option<i64> = None; 

    for piece in pieces {
        let value = piece.as_ref().parse::<i64>().expect("Parsing error");
        
        if previous_value.is_some() {
            let difference = value - previous_value.unwrap();

            // Check the increase/decrease amount bounds
            let abs_diff = difference.abs();
            if abs_diff > 3 || abs_diff < 1 {
                is_safe = false;
                break;
            }

            let is_increase = difference > 0;

            if should_increase.is_some() {
                // Check if the row is violating the increase/decrease rule
                if is_increase != should_increase.unwrap() {
                    is_safe = false;
                    break;
                }
            }

            should_increase = Option::from(is_increase);
        }

        previous_value = Option::from(value);
    }

    return is_safe;
}

fn evaluate_report_part2(line: &str) -> bool {

    let pieces: Vec<_> = line.split(' ').filter(|&x| !x.is_empty()).collect();
    
    let length = pieces.len();
    
    for skip_index in 0..length {
        let new_sequence = pieces.clone().into_iter().enumerate().filter(|&(i, _)| i != skip_index).map(|(_, v)| v);

        let is_safe = evaluate_report_part1(new_sequence);

        if is_safe {
            return true;
        }
    }

    return false; // no valid sequences
}