use std::collections::HashSet;
use std::io::Read;
use std::fs::File;
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
    let _test_content = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
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

    use std::time::Instant;
    let now = Instant::now();

    for part  in 1..3 {

        let mut grid: Vec<Vec<char>> = map_lines.iter().map(|x| x.chars().collect()).collect();

        if part == 2 {
            let mut scaled: Vec<Vec<char>> = Vec::with_capacity(grid.len());

            for line in grid.iter() {
                let mut grid_line: Vec<char> = Vec::with_capacity(line.len());
                for ch in line {
                    if *ch == '#' {
                        grid_line.push('#');
                        grid_line.push('#');
                    }
                    else if *ch == 'O' {
                        grid_line.push('[');
                        grid_line.push(']');
                    }
                    else if *ch == '.' {
                        grid_line.push('.');
                        grid_line.push('.');
                    }
                    else if *ch == '@' {
                        grid_line.push('@');
                        grid_line.push('.');
                    }
                }
                scaled.push(grid_line);
            }

            grid = scaled;
        }

        let robot_y = grid.iter().position(|line| line.iter().position(|c| *c == '@').is_some()).unwrap();
        let robot_x = grid[robot_y].iter().position(|c| *c == '@').unwrap();

        let mut robot = Vector2::new(robot_x as i32, robot_y as i32);

        // Mark the robot's position as empty
        grid[robot.y as usize][robot.x as usize] = '.';
        
        let width = grid[0].len();
        let height = grid.len();

        'DirectionLoop: for dir in directions.iter() {

            //print_grid(&grid, &robot, width, height);

            let direction = match dir {
                '<' => Vector2::West(),
                '>' => Vector2::East(),
                '^' => Vector2::North(),
                'v' => Vector2::South(),
                _ => panic!("Unknown input")
            };

            //println!("New direction: {0}", dir);

            let new_pos = robot.add(&direction);
            let new_pos_ch = get_char_safe(&grid, &new_pos, width, height);

            if new_pos_ch == '.' { // empty
                robot = new_pos;
                continue;
            }
            else if new_pos_ch == '[' || new_pos_ch == ']' || new_pos_ch == 'O' { // box
                
                // Accumulate blocked positions
                let mut move_stack: Vec<HashSet<Vector2>> = Vec::new();
                move_stack.push(HashSet::new());
                
                let mut box_positions = move_stack.last_mut().unwrap();

                if new_pos_ch == '[' {
                    box_positions.insert(new_pos.clone());
                    box_positions.insert(Vector2::new(new_pos.x + 1, new_pos.y));
                }
                else if new_pos_ch == ']' {
                    box_positions.insert(new_pos.clone());
                    box_positions.insert(Vector2::new(new_pos.x - 1, new_pos.y));
                }
                else if new_pos_ch == 'O' {
                    box_positions.insert(new_pos.clone());
                }

                // Check the location beyond the blocked positions
                while !box_positions.is_empty() {

                    let mut new_boxes = HashSet::new();

                    // Check each blocked pos to see if we can move it
                    for blocked_pos in box_positions.iter() {
                        let beyond_pos = blocked_pos.add(&direction);
                        if box_positions.contains(&beyond_pos) {
                            continue; // box cannot block itself
                        }
        
                        let ch = get_char_safe(&grid, &beyond_pos, width, height);
        
                        if ch == '#' {
                            continue 'DirectionLoop; // Hit a wall
                        }
                        else if ch == '[' { // got new boxes
                            new_boxes.insert(beyond_pos.clone());
                            new_boxes.insert(Vector2::new(beyond_pos.x + 1, beyond_pos.y));
                        }
                        else if ch == ']' { // got new boxes
                            new_boxes.insert(beyond_pos.clone());
                            new_boxes.insert(Vector2::new(beyond_pos.x - 1, beyond_pos.y));
                        }
                        else if ch == 'O' {
                            new_boxes.insert(beyond_pos.clone());
                        }
                    }

                    move_stack.push(new_boxes);
                    box_positions = move_stack.last_mut().unwrap();
                }

                // If we got here, we can push
                while !move_stack.is_empty() {
                    let mut positions = move_stack.pop().unwrap().iter().map(|x| x.clone()).collect::<Vec<_>>();

                    positions.sort_by(|a, b| a.x.cmp(&b.x));

                    if *dir == '>' {
                        positions.reverse();
                    }

                    for pos in positions {
                        let target_pos = pos.add(&direction);
                        grid[target_pos.y as usize][target_pos.x as usize] = grid[pos.y as usize][pos.x as usize];
                        grid[pos.y as usize][pos.x as usize] = '.';
                    }
                }

                robot = new_pos;
            }
            else { 
                // Wall, don't move
            }
        }

        let mut sum = 0;

        for y in 0..height {
            for x in 0..width {
                let ch = get_char_safe(&grid, &Vector2::new(x as i32, y as i32), width, height);
                if ch == '[' || ch == 'O' {
                    sum += y * 100 + x;
                }
            }
        }

        // Part 1 = 1318523
        // Part 2 = 1337648
        println!("[Part{part}]: Sum of GPS coords = {sum}"); 
    }

    let elapsed = now.elapsed();

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