use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines: Vec<_> = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect();

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    use std::time::Instant;
    let now = Instant::now();

    for line in lines {
        let pieces: Vec<_> = line.split(": ").collect();
        let left = pieces[0].parse::<u64>().expect("Not a number");
        let operands: Vec<_> = pieces[1].split(' ').map(|x| x.parse::<u64>().expect("not a number")).collect(); 

        if evalute_operands_part1(left, &operands) {
            //println!("{0} => {1:?} is VALID", left, operands);
            sum_part1 += left;
        }

        if evalute_operands_part2(left, &operands) {
            //println!("{0} => {1:?} is VALID", left, operands);
            sum_part2 += left;
        }
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Sum of valid test equations = {0}", sum_part1);
    println!("[Part2]: Sum of valid test equations = {0}", sum_part2);
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn evalute_operands_part1(left: u64, operands: &Vec<u64>) -> bool {

    let operator_count = u32::try_from(operands.len()-1).unwrap();
        
    for i in 0..u32::pow(2, operator_count)+1 {
        
        let mut sum = operands[0];

        // Apply operator combination to operands
        for bit in 0..operator_count {
            if i & u32::pow(2, bit) != 0 {
                sum += operands[(1 + bit) as usize];
            }
            else {
                sum *= operands[(1 + bit) as usize];
            }
        }

        if sum == left {
            return true;
        }
    }

    return false;
}

fn combinations(possibilities: u32, slots: u32)->Vec<String> {
    
    let mut result: Vec<String> = Vec::new();

    if slots == 1 {
        for p in 0..possibilities {
            result.push(p.to_string());
        }
    }
    else {
        for p in 0..possibilities {
            // Combine each value with the combinations of the subset
            for subcombo in combinations(possibilities, slots - 1) {
                result.push(p.to_string() + &subcombo);
            }
        }
    }

    result
}

fn evalute_operands_part2(left: u64, operands: &Vec<u64>) -> bool {

    let operator_count = u32::try_from(operands.len()-1).unwrap();

    for combo in combinations(3, operator_count) {
        let mut sum = operands[0];

        for digit in 0..combo.len() {
            sum = match combo.chars().nth(digit).unwrap() {
                '0' => { (sum.to_string() + &operands[(1 + digit) as usize].to_string()).parse::<u64>().unwrap() }
                '1' => { sum + operands[(1 + digit) as usize] }
                '2' => { sum * operands[(1 + digit) as usize] }
                _ => panic!()
            }
        }

        if sum == left {
            return true;
        }
    }

    return false;
}