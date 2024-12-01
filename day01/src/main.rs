use std::io::Read;
use std::fs::File;

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

    let mut file_content =String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n').filter(|&x| !x.is_empty());

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in lines {
        println!("{}", line);
        let pieces: Vec<_> = line.split(' ').filter(|&x| !x.is_empty()).collect();

        println!("First piece: {:?}", pieces[0]);
        println!("Second piece: {:?}", pieces[1]);

        left.push(pieces[0].parse::<i64>().unwrap());
        right.push(pieces[1].parse::<i64>().unwrap());

        println!("Left: {0}, Right: {1}", pieces[0].parse::<i64>().unwrap(), pieces[1].parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();
    let mut sum: i64 = 0;

    for i in 0..left.len() {
        println!("Distance between {0} and {1} is {2}", left[i], right[i], (right[i] - left[i]).abs());
        sum += (right[i] - left[i]).abs();
    }

    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    
    println!(" ---- ");
    println!("Sum = {:?}", sum);
    //println!("Data file: {0}", file_content);
}
