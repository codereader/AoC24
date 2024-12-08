use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines: Vec<_> = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect();

    let height = lines.len();
    let width = lines[0].len();

    use std::time::Instant;
    let now = Instant::now();

    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut unique_positions_part1: HashSet<(i32, i32)> = HashSet::new();
    let mut unique_positions_part2: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {

            let freq = lines[y].chars().nth(x).unwrap();
            if freq == '.' {
                continue;
            }

            let entry = antennae.entry(freq).or_insert_with(|| Vec::new());
            entry.push((x as i32, y as i32));
        }
    }

    // Part 1: Single Antinode generated in each direction
    generate_antinode_positions(&antennae, &mut unique_positions_part1, width, height, 1, false);

    // Part 2: Many Antinodes generated in each direction
    generate_antinode_positions(&antennae, &mut unique_positions_part2, width, height, 500000, true);

    let elapsed = now.elapsed();

    println!("[Part1]: Unique antinode locations = {0}", unique_positions_part1.len()); // 376
    println!("[Part2]: Unique antinode locations = {0}", unique_positions_part2.len()); // 1352
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn generate_antinode_positions(antennae: &HashMap<char, Vec<(i32, i32)>>, unique_positions: &mut HashSet<(i32, i32)>,
 width: usize, height: usize, steps: usize, include_antennae: bool) {
    for key_value in antennae {
        let positions = &key_value.1;

        // Combine all antennae with one of the others
        for n in 0..positions.len()-1 {

            let antenna1 = &positions[n];

            for k in n+1..positions.len() {
                let antenna2 = &positions[k];

                // Vector 1->2
                let distance = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

                if include_antennae {
                    // The two antennae are positions on their own, they are within the grid
                    unique_positions.insert(*antenna1);
                    unique_positions.insert(*antenna2);
                }
                
                let mut steps_remaining = steps;
                let mut candidate =  (antenna2.0 + distance.0, antenna2.1 + distance.1);
                while pos_within_grid(&candidate, width, height) && steps_remaining > 0 {
                    unique_positions.insert(candidate);
                    candidate = (candidate.0 + distance.0, candidate.1 + distance.1);
                    steps_remaining -= 1;
                }

                steps_remaining = steps;
                candidate = (antenna1.0 - distance.0, antenna1.1 - distance.1);
                while pos_within_grid(&candidate, width, height) && steps_remaining > 0 {
                    unique_positions.insert(candidate);
                    candidate = (candidate.0 - distance.0, candidate.1 - distance.1);
                    steps_remaining -= 1;
                }
            }
        }
    }
}

fn pos_within_grid(pos: &(i32, i32), width: usize, height: usize) -> bool {
    return pos.0 >= 0 && pos.0 < (width as i32) && pos.1 >= 0 && pos.1 < (height as i32);
}
