use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

fn part_one(input: &str) -> usize {
    let mut disk = build_disk(input);
    defrag(&mut disk);

    let mut result = 0;
    for (idx, block) in disk.iter().enumerate() {
        result += idx * *block as usize
    }

    result
}

fn build_disk(file_map: &str) -> Vec<isize> {
    let mut file_blocks: Vec<isize> = vec![];
    let mut is_file_block = true;
    let mut file_idx = 0;

    for block in file_map.chars() {
        let x = block.to_digit(10).unwrap();
        if is_file_block {
            for _ in 0..x {
                file_blocks.push(file_idx);
            }
            file_idx += 1;
            is_file_block = false;
        } else {
            for _ in 0..x {
                file_blocks.push(-1)
            }
            is_file_block = true;
        }
    }

    file_blocks
}

fn defrag(disk: &mut Vec<isize>) {
    let mut ptr = 0;

    'outer: for rev_idx in (0..disk.len()).rev() {
        if disk[rev_idx] != -1 {
            for idx in ptr..disk.len() {
                if disk[idx] == -1 {
                    disk[idx] = disk[rev_idx];
                    disk.pop();
                    ptr = idx + 1;
                    break;
                }
                if idx == rev_idx {
                    break 'outer;
                }
            }
        } else {
            disk.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("2333133121414131402");
        assert_eq!(result, 1928);
    }
}
