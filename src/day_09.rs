/**
   Turns the given input file into the expanded file block format
 */
fn preprocess(path: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(path).expect(path);
    let mut result: Vec<i32> = vec![];
    let chars: Vec<char> = contents.chars().collect();
    let mut current_id = 0;

    for i in 0..chars.len() {
        let block_size = chars[i].to_digit(10).expect("Should be a digit in base 10");
        let append;
        if i % 2 == 0 {
            append = current_id;
            current_id += 1;
        } else { 
            append = -1;
        }

        for _ in 0..block_size {
            result.push(append);
        }
    }

    result
}

/**
   Moves the blocks around with the given rules and then returns the new computed checksum
 */
pub fn part_1(path: &str) -> u128 {
    let mut storage = preprocess(path);

    let mut left_pointer = 0;
    let mut right_pointer = storage.len() - 1;

    let mut checksum: u128 = 0;

    while left_pointer < storage.len() {
        // In this case all blocks to the left are already filled, and all blocks
        // to the right should be empty. Just compute the checksum now.
        if left_pointer >= right_pointer {
            let block_value = storage[left_pointer];

            if block_value > 0 {
                checksum += (left_pointer as u128) * (storage[left_pointer] as u128);
            }

            left_pointer += 1;
            continue;
        }

        // If the left block is free and the right block is occupied, swap them.
        // In this case we also need to advance the right pointer
        if storage[left_pointer] < 0 {
            // Advance the right pointer until we find the next filled block
            while storage[right_pointer] < 0 {
                right_pointer -= 1;
            }

            storage[left_pointer] = storage[right_pointer];
            storage[right_pointer] = -1;
            right_pointer -= 1;
        }

        // At this point the block at left pointer is guaranteed to be filled, so add it to the checksum computation
        checksum += (left_pointer as u128) * (storage[left_pointer] as u128);

        // The left pointer is advanced in every iteration
        left_pointer += 1;
    }

    println!("Computed {} as checksum", checksum);
    checksum
}

pub fn part_2(path: &str) -> u128 {
    let mut storage = preprocess(path);

    let mut pointer = storage.len() as isize;
    let mut block_id: i32 = -1;

    while pointer > 0 {
        // Get the block size and block id
        while block_id < 0 && pointer >= 0 {
            pointer -= 1;
            block_id = storage[pointer as usize];
        }

        let block_start = pointer;
        let mut next_block_id = block_id;

        // As long as we are inside a block, advance the pointer
        while next_block_id == block_id && pointer > 0 {
            pointer -= 1;
            next_block_id = storage[pointer as usize];
        }

        let block_end = pointer;
        let block_size = block_start - block_end;

        // Check where the block fits beginning from the start
        let mut free_block_start = -1;
        let mut free_block_size = 0;
        for i in 0..=block_end {
            // If we hit a filled block, reset the counts and skip it
            if storage[i as usize] >= 0 {
                free_block_size = 0;
                free_block_start = -1;
                continue;
            }

            // We need to start a new free block count
            if free_block_start < 0 {
                free_block_size = 0;
                free_block_start = i as isize;
            }

            free_block_size += 1;

            // In this case, we hit a filled block. We check
            // if the found filled block from the end fits into
            // the last free block
            if block_size <= free_block_size {
                let mut f = free_block_start as usize;

                // The block end must be exclusive
                for b in block_end+1..=block_start {
                    storage[f] = storage[b as usize];
                    storage[b as usize] = -1;
                    f += 1;
                }

                // We break the loop because we successfully moved the block
                break;
            }
        }

        block_id = next_block_id;
    }

    let mut checksum = 0;

    for i in 0..storage.len() {
        if storage[i] >= 0 {
            checksum += (i as u128) * (storage[i] as u128);
        }
    }

    println!("Computed {} as checksum", checksum);
    checksum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_preprocess() {
        let result = preprocess("./inputs/day_09_test.txt");
        assert_eq!(result, vec![
            0, 0,
            -1, -1, -1,
            1, 1, 1,
            -1, -1, -1,
            2,
            -1, -1, -1,
            3, 3, 3,
            -1,
            4, 4,
            -1,
            5, 5, 5, 5,
            -1,
            6, 6, 6, 6,
            -1,
            7, 7, 7,
            -1,
            8, 8, 8, 8,
            9, 9,
        ]);
    }

    #[test]
    fn test_part_1() {
        let checksum = part_1("./inputs/day_09_test.txt");
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_part_2() {
        let checksum = part_2("./inputs/day_09_test.txt");
        assert_eq!(checksum, 2858);
    }
}