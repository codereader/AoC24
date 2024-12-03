use std::io::Read;
use std::fs::File;
use regex::Regex;

fn main() {
    // Create an empty mutable string
    let test_content = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");

    let expr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let results: Vec<_> = expr.captures_iter(normalized_file.as_str()).map(|m| m).collect();

    let mut sum: i64 = 0;
    for result in results {
        let a = &result[1].parse::<i64>().unwrap();
        let b = &result[2].parse::<i64>().unwrap();
        println!("{0} x {1} = {2}", a, b, a*b);
        sum += a*b;
    }

    println!("[Part1]: Sum of mul(a,b) = {0}", sum);
}
