use std::collections::HashSet;
use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines: Vec<Vec<char>> = normalized_file.split('\n').filter(|&x| !x.is_empty()).map(|x| x.chars().collect()).collect();

    let start_y = i32::try_from(lines.iter().enumerate().find_map(|(index, line)| if line.contains(&'^') { Some(index) } else { None }).expect("Cannot find start line")).expect("Cannot convert to i32");

    let x = lines[usize::try_from(start_y).unwrap()].iter().position(|&c| c == '^').unwrap();
    let start_x = i32::try_from(x).expect("Out of bounds");
    
    let width = i32::try_from(lines[0].len()).expect("Failed to convert");
    let height = i32::try_from(lines.len()).expect("Failed to convert");

    let mut pos = (start_x, start_y);
    let mut dir = (0, -1);

    use std::time::Instant;
    let now = Instant::now();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((pos.0, pos.1));

    let mut visited_directions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    visited_directions.insert((pos.0, pos.1, dir.0, dir.1));

    let mut blocker_positions: Vec<(i32, i32)> = Vec::new();

    loop {
        //print_grid(&pos, &lines, &visited_positions, &blocker_positions);

        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

        let next = get_pos_safe(&new_pos, &lines, width, height);

        if next == '\0' {
            break;
        }

        if next == '.' && !visited_positions.contains(&new_pos) {
            // Before moving forward, consider placing a blocker at new_pos, would this lead into a loop
            let right = (-dir.1, dir.0);

            // Evaluate this path
            let is_looping_path = evaluate_path(&pos, &right, &lines, &visited_directions, &new_pos);

            if is_looping_path {
                // By placing a blocker in the next square, we'd create a loop
                blocker_positions.push((new_pos.0, new_pos.1));
                //println!("Found blocker at {0},{1}", new_pos.0, new_pos.1);
            }
        }
        
        // Proceed
        if next == '#' {
            dir = (-dir.1, dir.0);
            //pos = pos + dir;
            continue;
        }
        else {
            pos = new_pos;
        }

        visited_positions.insert((pos.0, pos.1));
        visited_directions.insert((pos.0, pos.1, dir.0, dir.1));
    }

    let elapsed = now.elapsed();
    
    println!("[Part1]: Number of visited grid positions = {0}", visited_positions.len()); // 4647
    println!("[Part2]: Number of blocker positions = {0}", blocker_positions.len()); // 1723
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn evaluate_path(probe_pos: &(i32, i32), dir: &(i32, i32), lines: &Vec<Vec<char>>, parent_directions: &HashSet<(i32,i32,i32,i32)>, blocked_pos: &(i32, i32)) -> bool {

    let mut pos = probe_pos.clone();
    let mut dir = dir.clone();

    let mut visited_directions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    visited_directions.insert((pos.0, pos.1, dir.0, dir.1));

    let width = i32::try_from(lines[0].len()).expect("Failed to convert");
    let height = i32::try_from(lines.len()).expect("Failed to convert");

    loop {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

        let next = get_pos_safe(&new_pos, &lines, width,  height);

        if next == '\0' {
            return false;
        }

        if next == '#' || (new_pos.0 == blocked_pos.0 && new_pos.1 == blocked_pos.1) {
            dir = (-dir.1, dir.0);
        }
        else {
            pos = new_pos;
        }

        let entry = (pos.0, pos.1, dir.0, dir.1);
        if parent_directions.contains(&entry) || !visited_directions.insert(entry) {
            return true; // loop
        }
    }
}

/*
fn print_grid(pos: &(i32, i32), lines: &Vec<Vec<char>>, visited_positions: &HashSet<(i32, i32)>, blocker_positions: &Vec<(i32, i32)>) {
    let width = i32::try_from(lines[0].len()).expect("Failed to convert");
    let height = i32::try_from(lines.len()).expect("Failed to convert");
    
    for y in 0..height {
        for x in 0..width {
            if pos.0 == x && pos.1 == y {
                print!("^");
            }
            else if blocker_positions.contains(&(x,y)) {
                print!("O");
            }
            else if visited_positions.contains(&(x,y)) {
                print!("X");
            }
            else {
                print!("{}", get_pos_safe(&(x, y), lines));
            }
        }
        println!();
    }
    
    //println!("Current Position: {0},{1}", pos.0, pos.1);
    println!("_______________");
}
*/

fn pos_in_bounds(pos: &(i32, i32), width: i32, height: i32)->bool {
    if pos.0 >= width || pos.0 < 0 || pos.1 >= height || pos.1 < 0 {
        return false;
    }

    return true;
}

fn get_pos_safe(pos: &(i32, i32), lines: &Vec<Vec<char>>, width: i32, height: i32)->char {

    if !pos_in_bounds(pos, width, height) {
        return '\0';
    }

    return lines[usize::try_from(pos.1).unwrap()][usize::try_from(pos.0).unwrap()];
}

