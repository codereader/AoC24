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
    let _test_content = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines  = normalized_file.split('\n').collect::<Vec<_>>();

    let mut map_lines: Vec<_> = Vec::new();
    let mut i = 0;
    loop {
        if lines[i].is_empty() {
            break;
        }
        else {
            map_lines.push(lines[i]);
            i += 1;
        }
    }

    let dir_lines = lines.into_iter().skip(i).collect::<Vec<_>>();

    let directions = dir_lines.into_iter().map(|x| x.chars()).flatten().collect::<Vec<_>>();

    let mut grid: Vec<Vec<char>> = map_lines.into_iter().map(|x| x.chars().collect()).collect();
    let mut sum_part1 = 0;

    let width = grid[0].len();
    let height = grid.len();

    use std::time::Instant;
    let now = Instant::now();

    let robot_y = grid.iter().position(|line| line.iter().position(|c| *c == '@').is_some()).unwrap();
    let robot_x = grid[robot_y].iter().position(|c| *c == '@').unwrap();
    let mut robot = Vector2::new(robot_x as i32, robot_y as i32);

    println!("Robot at {0}|{1}", robot_x, robot_y);
    println!("Directions {:?}", directions);

    // Mark the robot's position as empty
    grid[robot.y as usize][robot.x as usize] = '.';

    'DirectionLoop: for dir in directions {

        //print_grid(&grid, &robot, width, height);

        let direction = match dir {
            '<' => Vector2::West(),
            '>' => Vector2::East(),
            '^' => Vector2::North(),
            'v' => Vector2::South(),
            _ => panic!("Unknown input")
        };

        let new_pos = robot.add(&direction);
        let new_pos_ch = get_char_safe(&grid, &new_pos, width, height);

        if new_pos_ch == '.' { // empty
            robot = new_pos;
            continue;
        }
        else if new_pos_ch == 'O' { // box
            
            let mut beyond_box = new_pos.add(&direction);

            while pos_within_grid(&beyond_box, width, height) {
                let ch = get_char_safe(&grid, &beyond_box, width, height);

                if ch == '.' {
                    break;
                }
                if ch == '#' {
                    continue 'DirectionLoop; // hit a wall
                }
                beyond_box = beyond_box.add(&direction);
            }

            if get_char_safe(&grid, &beyond_box, width, height) == '.' {
                // Move all boxes
                let opposite_direction = Vector2::new(-direction.x, -direction.y);
                let mut target_pos = beyond_box;
                let mut before_target_pos = target_pos.add(&opposite_direction);

                while before_target_pos != robot {
                    grid[target_pos.y as usize][target_pos.x as usize] = grid[before_target_pos.y as usize][before_target_pos.x as usize];
                    grid[before_target_pos.y as usize][before_target_pos.x as usize] = '.';
                    target_pos = target_pos.add(&opposite_direction);
                    before_target_pos = target_pos.add(&opposite_direction);
                }

                robot = new_pos;

                continue 'DirectionLoop;
            }
        }
        else { 
            // Wall, don't move
        }
    }

    print_grid(&grid, &robot, width, height);

    for y in 0..height {
        for x in 0..width {
            if get_char_safe(&grid, &Vector2::new(x as i32, y as i32), width, height) == 'O' {
                sum_part1 += y * 100 + x;
            }
        }
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Sum of GPS coords = {0}", sum_part1); // ???
    println!("[Part2]: ... = {0}", 0); // ???
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn print_grid(grid: &Vec<Vec<char>>, robot: &Vector2, width: usize, height: usize) {
    
    for y in 0..height {
        for x in 0..width {
            if robot.x == x as i32 && robot.y == y as i32 {
                print!("@");
                continue;
            }
            print!("{0}", get_char_safe(grid, &Vector2::new(x as i32, y as i32), width, height));
        }
        println!();
    }
    println!("_______________");
}

fn get_char_safe(grid: &Vec<Vec<char>>, pos: &Vector2, width: usize, height: usize) -> char {
    if !pos_within_grid(pos, width, height) {
        return '\0';
    }

    return grid[pos.y as usize][pos.x as usize];
}

fn pos_within_grid(pos: &Vector2, width: usize, height: usize) -> bool {
    return pos.x >= 0 && pos.x < (width as i32) && pos.y >= 0 && pos.y < (height as i32);
}