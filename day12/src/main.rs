use std::collections::HashSet;
use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    
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

    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    let mut areas: Vec<(usize, usize)> = Vec::new();

    let width = grid[0].len();
    let height = grid.len();

    for y in 0..height {
        for x in 0..width {
            let pos = (x as i32, y as i32);

            if occupied.contains(&pos) {
                continue;
            }
            
            // A new area to investigate
            let mut to_investigate: Vec<(i32, i32)> = Vec::new();
            to_investigate.push(pos);
    
            let mut area = 0;
            let mut perimeter = 0;
    
            while to_investigate.len() > 0 {
                let pos = to_investigate.pop().unwrap();
    
                if occupied.contains(&pos) {
                    continue;
                }

                let plant = grid[pos.1 as usize][pos.0 as usize];
    
                area += 1;
                occupied.insert(pos);
    
                // Check all four directions
                let north = (pos.0 + 0, pos.1 - 1);
                let south = (pos.0 + 0, pos.1 + 1);
                let east = (pos.0 + 1, pos.1 + 0);
                let west = (pos.0 - 1, pos.1 + 0);
    
                if get_plant_safe(&grid, &north, width, height) != plant {
                    perimeter += 1;
                }
                else if !occupied.contains(&north) {
                    to_investigate.push(north);
                }
    
                if get_plant_safe(&grid, &south, width, height) != plant {
                    perimeter += 1;
                }
                else if !occupied.contains(&south) {
                    to_investigate.push(south);
                }
    
                if get_plant_safe(&grid, &east, width, height) != plant {
                    perimeter += 1;
                }
                else if !occupied.contains(&east) {
                    to_investigate.push(east);
                }
    
                if get_plant_safe(&grid, &west, width, height) != plant {
                    perimeter += 1;
                }
                else if !occupied.contains(&west) {
                    to_investigate.push(west);
                }
            }
    
            areas.push((area, perimeter));
        }
    }

    //println!("{:?}", areas);

    for area in areas.iter() {
        sum_part1 += area.0 * area.1;
    }    

    let elapsed = now.elapsed();

    println!("[Part1]: Total cost = {0}", sum_part1); // ???
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn get_plant_safe(grid: &Vec<Vec<char>>, pos: &(i32, i32), width: usize, height: usize) -> char {
    if !pos_within_grid(pos, width, height) {
        return '\0';
    }

    return grid[pos.1 as usize][pos.0 as usize];
}

fn pos_within_grid(pos: &(i32, i32), width: usize, height: usize) -> bool {
    return pos.0 >= 0 && pos.0 < (width as i32) && pos.1 >= 0 && pos.1 < (height as i32);
}