use std::io::Read;
use std::fs::File;
use regex::Regex;

fn main() {
    let _test_content_part1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let _test_content_part2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let expr_part1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let results_part1: Vec<_> = expr_part1.captures_iter(file_content.as_str()).map(|m| m).collect();

    let mut sum_part1: i64 = 0;
    for result in results_part1 {
        let a = &result[1].parse::<i64>().unwrap();
        let b = &result[2].parse::<i64>().unwrap();
        //println!("{0} x {1} = {2}", a, b, a*b);
        sum_part1 += a*b;
    }

    println!("[Part1]: Sum of mul(a,b) = {0}", sum_part1); // 173529487

    let expr_part2 = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do(n't|)\(\))").unwrap();
    let results_part2: Vec<_> = expr_part2.captures_iter(&file_content).map(|m| m).collect();

    let mut sum_part2: i64 = 0;
    let mut enabled = true;
    for result in results_part2 {

        if (&result[0]).eq("do()") {
            //println!("Enabling");
            enabled = true;
        }
        else if (&result[0]).eq("don't()") {
            //println!("Disabling");
            enabled = false;
        }
        else if enabled {
            let a = &result[2].parse::<i64>().unwrap();
            let b = &result[3].parse::<i64>().unwrap();
            sum_part2 += a*b;
        }
    }

    println!("[Part2]: Sum of mul(a,b) with conditions = {0}", sum_part2); // 99532691
}
