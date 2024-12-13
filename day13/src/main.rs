use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::fs::File;
use std::hash::Hash;
use std::ops::Add;
use regex::Regex;
use std::cmp::min;

struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn add(&self, other: &Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn add_by_value(&self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn from(tuple: &(i32, i32)) -> Vector2 {
        return Vector2 { x: tuple.0, y: tuple.1 }; 
    }

    const fn North() -> Vector2 { return Vector2 { x: 0, y: -1 }; }
    const fn South() -> Vector2 { return Vector2 { x: 0, y: 1 }; }
    const fn East() -> Vector2 { return Vector2 { x: 1, y: 0 }; }
    const fn West() -> Vector2 { return Vector2 { x: -1, y: 0 }; }
}

impl<'a, 'b> Add<&'b Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn add(self, other: &'b Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b> Add<Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Hash for Vector2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Eq for Vector2 {}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
}

fn main() {
    // Create an empty mutable string
    let _test_content = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines  = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect::<Vec<_>>();

    use std::time::Instant;
    let now = Instant::now();

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    let price_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let num_machines = lines.len() / 3;
    println!("Machines: {num_machines}");

    for index in 0..num_machines {
        let button_a = button_regex.captures_iter(lines[index*3]).map(|m| m).next().unwrap();
        let button_b = button_regex.captures_iter(lines[index*3 + 1]).map(|m| m).next().unwrap();

        let price_capture = price_regex.captures_iter(lines[index*3 + 2]).map(|m| m).next().unwrap();

        let a = (&button_a[1].parse::<i64>().unwrap(), &button_a[2].parse::<i64>().unwrap());
        let b = (&button_b[1].parse::<i64>().unwrap(), &button_b[2].parse::<i64>().unwrap());
        let price = (&price_capture[1].parse::<i64>().unwrap(), &price_capture[2].parse::<i64>().unwrap());

        println!("A = {:?}, B = {:?}, P = {:?}", a, b, price);

        let max_a = min(price.0 / a.0, 100);
        let max_b = min(price.0 / b.0, 100);

        let mut solutions: Vec<i64> = Vec::new();
        for ax in 0..max_a+1 {
            for bx in 0..max_b+1  {
                if ax * a.0 + bx * b.0 == *price.0 && ax * a.1 + bx * b.1 == *price.1 {
                    println!("Solution: {0}xA {1}xB, Cost = {2}", ax, bx, ax * 3 + bx * 1);
                    solutions.push(ax * 3 + bx * 1);
                }
            }
        }

        if solutions.len() == 0 {
            println!("No solution");
        }
        else {
            if solutions.len() > 1 {
                println!("More than one solution");
            }
            solutions.sort();
            let min_cost = solutions.iter().next().unwrap();
            sum_part1 += min_cost;
            println!("Minimum cost: {0}", min_cost);
        }
    }


    let elapsed = now.elapsed();

    println!("[Part1]: Minimum cost = {0}", sum_part1); // ???
    println!("[Part2]: ... = {0}", sum_part2); // ???
    println!("Elapsed Time: {:.2?}", elapsed);
}
