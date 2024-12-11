use std::collections::HashMap;
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
    let stones: Vec<_> = normalized_file.split(' ').filter(|&x| !x.is_empty()).map(|x| x.parse::<u64>().expect("Parse error")).collect();

    use std::time::Instant;
    let now = Instant::now();
    let mut sum_part1 = 0;

    let mut known_sums: HashMap<(u64, u32), usize> = HashMap::new();

    // For every number, enter a recursive calculator
    for i in 0..stones.len() {
        let sum_for_i = calculate_sum(stones[i], 25, &mut known_sums);
        sum_part1 += sum_for_i;
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Number of stones after 25 blinks = {0}", sum_part1); // 183248
    println!("Elapsed Time: {:.2?}", elapsed);

    let now = Instant::now();

    // Part 2: do it recursively with mementos
    let mut sum_part2 = 0;

    // For every number, enter a recursive calculator
    for i in 0..stones.len() {
        let sum_for_i = calculate_sum(stones[i], 75, &mut known_sums);
        sum_part2 += sum_for_i;
    }

    let elapsed = now.elapsed();

    println!("[Part2]: Number of stones after 75 blinks = {0}", sum_part2); // 218811774248729
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn calculate_sum(number: u64, steps: u32, known_sums: &mut HashMap<(u64, u32), usize>) -> usize {

    let known_sum = known_sums.get(&(number, steps));

    if known_sum.is_none() {
        // Calculate and write back to known sums
        let mut sub_sum = 0;
        let subset = run_step(number);

        for subset_element in subset {
            sub_sum += if steps > 1 {
                calculate_sum(subset_element, steps - 1, known_sums)
            } else {
                1
            };
        }
        
        known_sums.insert((number, steps), sub_sum);
        return sub_sum;
    }

    return known_sum.cloned().unwrap();
}

fn run_step(number: u64) -> Vec<u64> {
    if number == 0 {
        return vec!(1);
    }
    
    let stone_string = number.to_string();

    if stone_string.len() % 2 == 0 {
        let num1 = stone_string[..(stone_string.len() / 2)].parse::<u64>().expect("Failed to parse new number");
        let num2 = stone_string[(stone_string.len() / 2)..].parse::<u64>().expect("Failed to parse new number");
        return vec!(num1, num2);
    }

    return vec!(number * 2024);
}