use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "125 17";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let mut stones: Vec<_> = normalized_file.split(' ').filter(|&x| !x.is_empty()).map(|x| x.parse::<u64>().expect("Parse error")).collect();

    use std::time::Instant;
    let now = Instant::now();

    for _ in 0..25 {

        let mut new_stones: Vec<u64> = Vec::with_capacity(stones.len() * 2);
        let mut stone_index = 0;
        while stone_index < stones.len() {
            
            if stones[stone_index] == 0 {
                new_stones.push(1);
                stone_index += 1;
                continue;
            }
            
            let stone_string = stones[stone_index].to_string();
            if stone_string.len() % 2 == 0 {
                let num1 = stone_string[..(stone_string.len() / 2)].parse::<u64>().expect("Failed to parse new number");
                let num2 = stone_string[(stone_string.len() / 2)..].parse::<u64>().expect("Failed to parse new number");
                new_stones.push(num1);
                new_stones.push(num2);
                stone_index += 1;
                continue;
            }

            new_stones.push(stones[stone_index] * 2024);
            stone_index += 1;
        }

        stones = new_stones;     
    }

    //println!("{:?}", stones);

    let elapsed = now.elapsed();

    println!("[Part1]: Number of stones after 25 blinks = {0}", stones.len()); // ???
    println!("[Part2]: ... = {0}", 0); // ???
    println!("Elapsed Time: {:.2?}", elapsed);
}
