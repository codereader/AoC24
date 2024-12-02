use std::io::Read;
use std::fs::File;

fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();

/*
// Create an empty mutable string
let mut file_content = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
*/

    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n').filter(|&x| !x.is_empty());

    let mut safe_report_count = 0;

    for line in lines {
        let pieces = line.split(' ').filter(|&x| !x.is_empty());

        let mut should_increase: Option<bool> = None;
        let mut is_safe = true;
        let mut previous_value: Option<i64> = None; 

        for piece in pieces {
            let value = piece.parse::<i64>().expect("Parsing error");
            
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

        if is_safe {
            safe_report_count += 1;
        }
    }

    println!("[Part1]: Safe report count = {}", safe_report_count);

}
