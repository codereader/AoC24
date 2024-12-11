use std::collections::{HashMap, HashSet};
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
    let mut sum_part1 = 0;

    let mut seen_numbers: HashSet<u64> = HashSet::new();

    for step in 0..25 {

        let mut new_stones: Vec<u64> = Vec::with_capacity(stones.len() * 2);
        let mut stone_index = 0;
        while stone_index < stones.len() {
            
            if stones[stone_index] == 0 {
                new_stones.push(1);
                stone_index += 1;
                seen_numbers.insert(1);
                continue;
            }
            
            let stone_string = stones[stone_index].to_string();
            if stone_string.len() % 2 == 0 {
                let num1 = stone_string[..(stone_string.len() / 2)].parse::<u64>().expect("Failed to parse new number");
                let num2 = stone_string[(stone_string.len() / 2)..].parse::<u64>().expect("Failed to parse new number");
                new_stones.push(num1);
                new_stones.push(num2);
                seen_numbers.insert(num1);
                seen_numbers.insert(num2);
                stone_index += 1;
                continue;
            }

            let num = stones[stone_index] * 2024;
            new_stones.push(num);
            seen_numbers.insert(num);
            stone_index += 1;
        }

        stones = new_stones; 
    }

    sum_part1 = stones.len();

    // Part 2: do it recursively with mementos
    let mut known_sums: HashMap<(u64, u32), usize> = HashMap::new();
    let mut sum_part2 = 0;

    let stones: Vec<_> = normalized_file.split(' ').filter(|&x| !x.is_empty()).map(|x| x.parse::<u64>().expect("Parse error")).collect();
    let steps_to_calculate = 75;

    // For every number, enter a recursive calculator
    for i in 0..stones.len() {
        let sum_for_i = calculate_sum(stones[i], steps_to_calculate, &mut known_sums);
        sum_part2 += sum_for_i;
    }

    let elapsed = now.elapsed();

    println!("Seen numbers count = {0}", seen_numbers.len());

    println!("[Part1]: Number of stones after 25 blinks = {0}", sum_part1); // 183248
    println!("[Part2]: Number of stones after 75 blinks = {0}", sum_part2); // 218811774248729
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn calculate_sum(number: u64, steps: u32, known_sums: &mut HashMap<(u64, u32), usize>) -> usize {

    if steps == 0 {
        return 1;
    }

    let known_sum = known_sums.get(&(number, steps));

    if known_sum.is_none() {
        // Calculate and write back to known sums
        let mut sub_sum = 0;
        let subset = run_step(number);

        for subset_element in subset {
            let sum = calculate_sum(subset_element, steps - 1, known_sums);
            sub_sum += sum;
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