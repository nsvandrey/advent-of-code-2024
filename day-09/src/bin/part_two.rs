use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part_two(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

fn part_two(input: &str) -> usize {
    let mut disk = build_disk(input);
    defrag(&mut disk);

    let mut result = 0;
    let mut idx = 0;
    for block in disk.iter() {
        if block.0 != -1 {
            for _ in 0..block.1 {
                result += idx * block.0 as usize;
                idx += 1;
            }
        } else {
            idx += block.1;
        }
    }

    result
}

fn build_disk(file_map: &str) -> Vec<(isize, usize, bool)> {
    let mut file_blocks: Vec<(isize, usize, bool)> = vec![];
    let mut is_file_block = true;
    let mut file_idx = 0;

    for block in file_map.chars() {
        let x = block.to_digit(10).unwrap() as usize;
        if is_file_block {
            file_blocks.push((file_idx, x, false));
            file_idx += 1;
            is_file_block = false;
        } else {
            file_blocks.push((-1, x, false));
            is_file_block = true;
        }
    }

    file_blocks
}

fn defrag(disk: &mut Vec<(isize, usize, bool)>) {
    let mut gidx = disk.len() - 1;

    while gidx != 0 {
        //for gidx in (0..disk.len()).rev() {
        if disk[gidx].0 != -1 && disk[gidx].2 == false {
            let file_block = (disk[gidx].0, disk[gidx].1, true);
            for idx in 0..gidx {
                if disk[idx].0 == -1 {
                    if disk[idx].1 == file_block.1 {
                        disk[idx] = file_block;
                        disk[gidx] = (-1, file_block.1, false);
                        gidx -= 1;
                        break;
                    }

                    if disk[idx].1 > file_block.1 {
                        let free_space_block = disk[idx];
                        disk[idx] = file_block;
                        let new_block = (-1, free_space_block.1 - file_block.1, false);
                        disk[gidx] = (-1, file_block.1, false);
                        disk.insert(idx + 1, new_block);
                        break;
                    }
                }
            }
            gidx -= 1;
        } else {
            gidx -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two("2333133121414131402");
        assert_eq!(result, 2858);
    }
}
