use std::io::Read;
use std::fs::File;
use std::convert::TryFrom;

fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();

/* 
// Create an empty mutable string
let mut file_content = "3   4
4   3
2   5
1   3
3   9
3   3";
*/

    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n').filter(|&x| !x.is_empty());

    // Parse the lines into two lists, left and right
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in lines {
        let mut pieces = line.split(' ').filter(|&x| !x.is_empty());

        left.push(pieces.next().expect("No left value").parse::<i64>().unwrap());
        right.push(pieces.next().expect("No right value").parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();

    let part1_sum: i64 = left.iter().zip(right.iter()).map(|(l, r)| (r - l).abs()).sum();

    println!("[Part 1] Sum = {:?}", part1_sum); // 1651298

    let mut part2_sum = 0;

    // Check how often each number in the left list occurs in the right one, then multiply it
    for l in left.iter() {
        let count = right.iter().filter(|&x| x == l).count();
        part2_sum += l * i64::try_from(count).expect("Cannot convert this value");
    }

    // Sum over all multiplications
    println!("[Part 2] Sum = {:?}", part2_sum); // 21306195
}
