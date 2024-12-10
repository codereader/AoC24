use std::io::Read;
use std::fs::File;

fn main() {
    let _test_content = "2333133121414131402";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let content = file_content.replace('\r', "");
    //let lines: Vec<_> = normalized_file.split('\n').filter(|&x| !x.is_empty()).collect();

    use std::time::Instant;
    let now = Instant::now();

    let chars: Vec<_> = content.chars().collect();
    
    let num_blocks = content.len() / 2 + content.len() % 2;
    println!("We have {0} blocks", num_blocks);

    let mut consume_block_index = num_blocks - 1;
    let mut consume_block_id = consume_block_index;
    let mut consume_block_width = chars[consume_block_index * 2].to_string().parse::<usize>().unwrap();

    let mut checksum: usize = 0;
    let mut current_pos = 0;

    for pos in 0..chars.len() {
        let block_len = chars[pos].to_string().parse::<usize>().unwrap();

        if pos >= consume_block_index * 2 {
            // Consider the rest of the consume_block
            for _ in 0..consume_block_width {
                checksum += consume_block_id * current_pos;
                current_pos += 1;
            }
            break;
        }

        if pos % 2 == 0 {
            // This is a block that has already been there at start, get its length
            let block_id = pos / 2;

            for _ in 0..block_len {
                checksum += block_id * current_pos;
                current_pos += 1;
            }
        }
        else {
            // This is free space, take from consume_block until the space has been exhausted
            let mut free_space_width = block_len;

            loop {
                if free_space_width == 0 {
                    break;
                }

                // Consume, if possible
                if consume_block_width == 0 {
                    // Proceed to the next block from the right, if allowed
                    consume_block_index -= 1;

                    if pos >= consume_block_index * 2 {
                        break; // no further blocks, skip this
                    }
                    consume_block_id = consume_block_index;
                    consume_block_width = chars[consume_block_index * 2].to_string().parse::<usize>().unwrap();
                }

                // Take one from the consume block, adding to the free space, add to checksum
                free_space_width -= 1;
                consume_block_width -= 1;
                checksum += current_pos * consume_block_id;
                current_pos += 1;
            }
        }
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Checksum = {0}", checksum); // 6154342787400
    println!("[Part2]: ... = {0}", 0); // ???
    println!("Elapsed Time: {:.2?}", elapsed);
}

