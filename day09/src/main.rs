use std::io::Read;
use std::fs::File;

struct Block {
    occupied: bool,
    file_id: i32,
    length: usize,
}

fn main() {
    let _test_content = "2333133121414131402";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let content = file_content.replace('\r', "");

    use std::time::Instant;
    let now = Instant::now();

    let chars: Vec<_> = content.chars().collect();
    
    let num_blocks = content.len() / 2 + content.len() % 2;
    println!("We have {0} blocks", num_blocks);

    // Part 1: Calculate the checksum from the left, streaming blocks from the far right into free space
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

    // Part 2: Parse the block info into a linked list to be able to move them as a whole
    let mut blocks: Vec<Block> = Vec::new();

    for pos in 0..chars.len() {
        blocks.push(Block {
            length: chars[pos].to_string().parse::<usize>().expect("Parse error"),
            occupied: pos % 2 == 0,
            file_id: if pos % 2 == 0 { i32::try_from(pos).unwrap() / 2 } else { -1 },
        });
    }

    let mut move_block_index = blocks.len();

    while move_block_index > 0 {

        move_block_index -= 1;
        
        if !blocks[move_block_index].occupied {
            continue;
        }

        for target_block_index in 0..move_block_index {

            if !blocks[target_block_index].occupied && blocks[target_block_index].length >= blocks[move_block_index].length {
                blocks[target_block_index].occupied = true;
                blocks[target_block_index].file_id = blocks[move_block_index].file_id;
                blocks[move_block_index].occupied = false;
                blocks[move_block_index].file_id = -1;

                let remaining_space = blocks[target_block_index].length - blocks[move_block_index].length;

                // The target block length might change
                blocks[target_block_index].length = blocks[move_block_index].length;

                if remaining_space > 0 {
                    blocks.insert(target_block_index + 1, Block {
                        occupied: false,
                        file_id: -1,
                        length: remaining_space,
                    });
                    move_block_index += 1; // everything has moved to the right by 1
                }
                break;
            }
        }
    }

    /* 
    for b in 0..blocks.len() {
        let block = blocks.get(b).unwrap();

        for _ in 0..block.length {
            print!("{}", if block.occupied { block.file_id.to_string() } else { String::from(".") });
        }
    }

    println!();
    */

    let mut checksum_part2: usize = 0;
    let mut current_pos = 0;

    for b in 0..blocks.len() {
        let block = blocks.get(b).unwrap();

        if !block.occupied {
            current_pos += block.length;
            continue;
        }

        for _ in 0..block.length {
            checksum_part2 += current_pos * (block.file_id as usize);
            current_pos += 1
        }
    }

    let elapsed = now.elapsed();

    println!("[Part1]: Checksum = {0}", checksum); // 6154342787400
    println!("[Part2]: Checksum = {0}", checksum_part2); // 6183632723350
    println!("Elapsed Time: {:.2?}", elapsed);
}

