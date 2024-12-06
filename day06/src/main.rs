use std::collections::HashSet;
use std::io::Read;
use std::fs::File;
use ico_math::Vector2Int;

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
    let lines: Vec<_> = normalized_file.split('\n').collect();

    let mut sum_part1 = 0;

    let width = i32::try_from(lines[0].len()).expect("Failed to convert");
    let height = i32::try_from(lines.len()).expect("Failed to convert");

    let start_y = i32::try_from(lines.iter().enumerate().find_map(|(index, line)| if line.find('^').is_some() { Some(index) } else { None }).expect("Cannot find start line")).expect("Cannot convert to i32");
    let start_x = i32::try_from(lines[usize::try_from(start_y).expect("Cannot convert to i32")].find('^').expect("Cannot find start position")).expect("Cannot convert position index");
    
    let mut pos = Vector2Int::new(start_x, start_y);
    let mut dir = Vector2Int::new(0, -1);

    println!("{0}|{1}", pos.x().value(), pos.y().value());

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    visited_positions.insert((pos.x().value(), pos.y().value()));

    loop {
        let new_pos = pos + dir;

        if new_pos.x().value() >= width || new_pos.x().value() < 0 ||
           new_pos.y().value() >= height || new_pos.y().value() < 0 {
            break;
        }

        let next = lines[usize::try_from(new_pos.y().value()).unwrap()].chars().nth(usize::try_from(new_pos.x().value()).unwrap()).unwrap();
        
        if next == '#' {
            dir = Vector2Int::new(-dir.y().value(), dir.x().value());
            pos = pos + dir;
        }
        else {
            pos = new_pos;
        }

        visited_positions.insert((pos.x().value(), pos.y().value()));
    }

    println!("[Part1]: Number of visited grid positions = {0}", visited_positions.len()); // 4647
}

