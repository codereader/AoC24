use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::fs::File;
use std::hash::Hash;
use std::ops::Add;

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
    let _test_content = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let grid: Vec<Vec<char>> = normalized_file.split('\n').filter(|&x| !x.is_empty()).map(|x| x.chars().collect()).collect();

    use std::time::Instant;
    let now = Instant::now();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let mut investigated: HashSet<Vector2> = HashSet::new();
    let mut areas: Vec<(usize, usize)> = Vec::new();
    let mut areas_part2: Vec<(usize, usize)> = Vec::new();

    let width = grid[0].len();
    let height = grid.len();

    const NORTH: Vector2 = Vector2::North();
    const SOUTH: Vector2 = Vector2::South();
    const EAST: Vector2 = Vector2::East();
    const WEST: Vector2 = Vector2::West();

    for y in 0..height {
        for x in 0..width {
            let pos = Vector2::from(&(x as i32, y as i32));

            if investigated.contains(&pos) {
                continue;
            }
            
            // A new area to investigate
            let mut to_investigate: Vec<Vector2> = Vec::new();
            to_investigate.push(pos);
    
            let mut area = 0;
            let mut perimeter = 0;
            let mut sides: HashMap<Vector2, HashSet<Vector2>> = HashMap::new();

            while to_investigate.len() > 0 {
                let pos = to_investigate.pop().unwrap();
    
                if investigated.contains(&pos) {
                    continue;
                }

                let plant = grid[pos.y as usize][pos.x as usize];
    
                area += 1;
                investigated.insert(pos.clone());

                // Check all four directions
                for side in vec!(NORTH, SOUTH, EAST, WEST) {
                    let new_pos = pos.add(&side);

                    if get_plant_safe(&grid, &new_pos, width, height) != plant {
                        perimeter += 1;
                        sides.entry(side).or_insert_with(|| HashSet::new()).insert(pos.clone());
                    }
                    else if !investigated.contains(&new_pos) {
                        to_investigate.push(new_pos);
                    }
                }
            }
    
            areas.push((area, perimeter));

            let mut num_sides = 0;

            for side in vec!(NORTH, SOUTH, EAST, WEST) {

                let is_north_or_south = side == NORTH || side == SOUTH;

                // Distinct y coords for N/S, distinct x coords for E/W
                let distinct_coords = sides.get(&side).unwrap().iter().map(|vec| if is_north_or_south { vec.y } else { vec.x }).collect::<HashSet<_>>();
                
                for i in distinct_coords {
                    // Get all x coordinates for this y coordinate (in case of N/S)
                    let mut all_coords = sides.get(&side).unwrap().iter().filter(|&vec | if is_north_or_south { vec.y == i } else { vec.x == i })
                        .map(|vec| if is_north_or_south { vec.x } else { vec.y }).collect::<Vec<_>>();
                    all_coords.sort();
                    
                    num_sides += 1;
                    for i in 1..all_coords.len() {
                        if all_coords[i] != all_coords[i-1] + 1 {
                            num_sides += 1;
                        }
                    }
                }
            }

            areas_part2.push((area, num_sides));
        }
    }

    for area in areas.iter() {
        sum_part1 += area.0 * area.1;
    }    

    for area in areas_part2.iter() {
        sum_part2 += area.0 * area.1;
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Total cost = {0}", sum_part1); // 1533024
    println!("[Part2]: Discount cost = {0}", sum_part2); // 910066
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn get_plant_safe(grid: &Vec<Vec<char>>, pos: &Vector2, width: usize, height: usize) -> char {
    if !pos_within_grid(pos, width, height) {
        return '\0';
    }

    return grid[pos.y as usize][pos.x as usize];
}

fn pos_within_grid(pos: &Vector2, width: usize, height: usize) -> bool {
    return pos.x >= 0 && pos.x < (width as i32) && pos.y >= 0 && pos.y < (height as i32);
}