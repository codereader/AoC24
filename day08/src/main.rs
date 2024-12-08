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
    let mut unique_positions: HashSet<(i32, i32)> = HashSet::new();

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

    //println!("{:?}", antennae);

    for key_value in antennae {
        let positions = &key_value.1;

        // Combine all antennae with one of the others
        for n in 0..positions.len()-1 {

            let antenna1 = &positions[n];

            for k in n+1..positions.len() {
                let antenna2 = &positions[k];

                //println!("Combining {0} with {1}", n, k);

                let distance = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);
                
                let antinode1_pos = (antenna1.0 + distance.0, antenna1.1 + distance.1);
                let antinode2_pos = (antenna2.0 - distance.0, antenna2.1 - distance.1);

                if pos_within_grid(&antinode1_pos, width, height) {
                    unique_positions.insert(antinode1_pos);
                }
                if pos_within_grid(&antinode2_pos, width, height) {
                    unique_positions.insert(antinode2_pos);
                }
            }
        }
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Unique antinode locations = {0}", unique_positions.len());
    println!("Elapsed Time: {:.2?}", elapsed);
}

fn pos_within_grid(pos: &(i32, i32), width: usize, height: usize) -> bool {
    return pos.0 >= 0 && pos.0 < (width as i32) && pos.1 >= 0 && pos.1 < (height as i32);
}
