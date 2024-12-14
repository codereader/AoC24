use std::collections::HashSet;
use std::io::{BufWriter, Read, Write};
use std::fs::{self, File};
use regex::Regex;
use std::cmp::min;
use std::hash::Hash;

use std::ops::Add;

#[derive(Debug)]
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

    fn new(x: i32, y: i32) -> Vector2 {
        return Vector2 { x,  y }; 
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

struct Robot {
    position: Vector2,
    velocity: Vector2
}

fn main() {
    // Create an empty mutable string
    let _test_content = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines  = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect::<Vec<_>>();

    use std::time::Instant;
    let now = Instant::now();

    let mut sum_part2 = 0;

    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=([\-\d]+),([\-\d]+)").unwrap();

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    let mut sum_quadrant1 = 0;
    let mut sum_quadrant2 = 0;
    let mut sum_quadrant3 = 0;
    let mut sum_quadrant4 = 0;

    for line in lines.iter() {
        let robot_data = robot_regex.captures_iter(line).map(|m| m).next().unwrap();

        let robot = Robot {
            position: Vector2::from(&(robot_data[1].parse::<i32>().unwrap(), robot_data[2].parse::<i32>().unwrap())),
            velocity: Vector2::from(&(robot_data[3].parse::<i32>().unwrap(), robot_data[4].parse::<i32>().unwrap()))
        };

        //println!("{:?}, {:?}", robot.position, robot.velocity);
        const STEPS: i32 = 100;

        let final_pos = Vector2::new(
             (robot.position.x + robot.velocity.x * STEPS + STEPS * WIDTH) % WIDTH,
             (robot.position.y + robot.velocity.y * STEPS + STEPS * HEIGHT) % HEIGHT);

        println!("{:?}", final_pos);

        if final_pos.x > WIDTH / 2 {
            if final_pos.y > HEIGHT / 2 {
                sum_quadrant2 += 1;
            }
            else if final_pos.y < HEIGHT / 2 {
                sum_quadrant1 += 1;
            }
        }
        else if final_pos.x < WIDTH / 2 {
            if final_pos.y > HEIGHT / 2 {
                sum_quadrant4 += 1;
            }
            else if final_pos.y < HEIGHT / 2 {
                sum_quadrant3 += 1;
            }
        }
    }

    let mut robots: Vec<Robot> = Vec::new();

    for line in lines {
        let robot_data = robot_regex.captures_iter(line).map(|m| m).next().unwrap();

        let robot = Robot {
            position: Vector2::from(&(robot_data[1].parse::<i32>().unwrap(), robot_data[2].parse::<i32>().unwrap())),
            velocity: Vector2::from(&(robot_data[3].parse::<i32>().unwrap(), robot_data[4].parse::<i32>().unwrap()))
        };
        robots.push(robot);
    }

    let mut step = 6667;
    let mut final_positions: HashSet<Vector2> = HashSet::new();
    let f = File::create("output.txt").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    
    loop {
        step += 1;
        if step % 10000 == 0 {
            println!("Step {}", step);
        }
        final_positions.clear();

        for i in 0..robots.len() {
            let final_pos = Vector2::new(
                (robots[i].position.x + robots[i].velocity.x * step + step * WIDTH) % WIDTH,
                (robots[i].position.y + robots[i].velocity.y * step + step * HEIGHT) % HEIGHT);

            final_positions.insert(final_pos);
        }

        for y in 0..HEIGHT {
            let mut line = String::with_capacity((WIDTH as usize) + 1);
            for x in 0..WIDTH {
                if final_positions.contains(&Vector2 { x, y }) {
                    line.push('X');
                }
                else {
                    line.push('.');
                }
            }
            line.push('\n');
            f.write_all(line.as_bytes()).expect("Unable to write data");
        }

        f.write(&vec!('\n' as u8)).expect("Unable to write data");
        
        if step > 6667 {
            break;
        }
    }

    let sum_part1 = sum_quadrant1 * sum_quadrant2 * sum_quadrant3 * sum_quadrant4;

    let elapsed = now.elapsed();

    println!("[Part1]: Number of robots in quadrants multiplied = {0}", sum_part1); // 230461440
    println!("[Part2]: Christmas tree at = {0}", step); // 6668
    println!("Elapsed Time: {:.2?}", elapsed);
}
