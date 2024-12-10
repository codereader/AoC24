use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use itertools::Itertools;

fn main() {
    // Create an empty mutable string
    let _test_content = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let grid: Vec<Vec<char>> = normalized_file.split('\n').filter(|&x| !x.is_empty()).map(|x| x.chars().collect()).collect();

    use std::time::Instant;
    let now = Instant::now();

    let width = grid[0].len();
    let height = grid.len();

    let mut paths_to_investigate: Vec<Vec<(i32, i32)>> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '0' {
                let mut path: Vec<(i32, i32)> = Vec::new();
                path.push((x as i32, y as i32));
                paths_to_investigate.push(path);
            }
        }
    }

    let mut trails: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    while paths_to_investigate.len() > 0 {
        let path = paths_to_investigate.pop().expect("Logic error");
        
        let next_height = path.len().to_string().chars().nth(0).unwrap();

        // Check surroundings
        let last_pos = path.last().unwrap().clone();

        investigate_position(&grid, (last_pos.0, last_pos.1 - 1), next_height, &path, &mut trails, &mut paths_to_investigate);
        investigate_position(&grid, (last_pos.0, last_pos.1 + 1), next_height, &path, &mut trails, &mut paths_to_investigate);
        investigate_position(&grid, (last_pos.0 + 1, last_pos.1), next_height, &path, &mut trails, &mut paths_to_investigate);
        investigate_position(&grid, (last_pos.0 - 1, last_pos.1), next_height, &path, &mut trails, &mut paths_to_investigate);
    }

    //println!("{:?}", trails);

    let elapsed = now.elapsed();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    for set in trails.values().into_iter() {
        sum_part1 += set.iter().unique().count();
    }

    for set in trails.values().into_iter() {
        sum_part2 += set.len();
    }

    println!("[Part1]: Sum of trailhead scores = {0}", sum_part1); // 754
    println!("[Part2]: Sum trailhead rating = {0}", sum_part2); // 1609
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn investigate_position(grid: &Vec<Vec<char>>, pos: (i32, i32), next_height: char, path: &Vec<(i32, i32)>, trails: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, paths_to_investigate: &mut Vec<Vec<(i32, i32)>>) {
    if get_grid_value_safe(grid, pos.0, pos.1) == next_height {
        let mut new_path = path.clone();
        new_path.push((pos.0, pos.1));

        if new_path.len() == 10 {
            let path_start = (path[0].0, path[0].1);

            trails.entry(path_start).or_insert_with(|| Vec::new()).push((pos.0, pos.1));
        }
        else {
            paths_to_investigate.push(new_path);
        }
    }
}

fn get_grid_value_safe(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {

    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return '\0';
    }

    return grid[y as usize][x as usize];
}