use std::{cmp::Ordering, collections::VecDeque, time::Instant};

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let input = &input[0];
    let mut files = Vec::new();
    let mut spaces = VecDeque::new();
    let mut disk_size: usize = 0;

    for (i, c) in input.chars().enumerate() {
        let block_length = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            // is file
            let file_id = i / 2;
            files.push((disk_size, block_length, file_id));
        } else {
            spaces.push_back((disk_size, block_length));
            // is empty space
        }
        disk_size += block_length;
    }

    let mut first_space = spaces.pop_front().unwrap();
    let mut last_file = files.pop().unwrap();
    let mut moved_files = Vec::new();

    loop {
        if first_space.0 > last_file.0 {
            files.push(last_file);
            break;
        }
        match first_space.1.cmp(&last_file.1) {
            Ordering::Greater => {
                // the first space is bigger, the whole file will go in it
                // ... 22 => 22.
                last_file.0 = first_space.0;
                first_space.0 += last_file.1;
                first_space.1 -= last_file.1;
                moved_files.push(last_file);
                if let Some(next_last_file) = files.pop() {
                    last_file = next_last_file;
                } else {
                    break;
                }
            }
            Ordering::Less => {
                // the last file is bigger, break it up and fill space
                let moved_chunk = (first_space.0, first_space.1, last_file.2);
                moved_files.push(moved_chunk);

                last_file.1 -= first_space.1;

                if let Some(next_first_space) = spaces.pop_front() {
                    first_space = next_first_space;
                } else {
                    files.push(last_file);
                    break;
                }
            }
            Ordering::Equal => {
                //  they are the same size, remove both
                last_file.0 = first_space.0;
                moved_files.push(last_file);

                if let (Some(next_first_space), Some(next_last_file)) =
                    (spaces.pop_front(), files.pop())
                {
                    first_space = next_first_space;
                    last_file = next_last_file;
                } else {
                    break;
                }
            }
        }
    }

    let checksum = calculate_checksum(files.into_iter().chain(moved_files));

    Ok(checksum)
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    let bench_start = Instant::now();
    let input = &input[0];
    let mut files = Vec::new();
    let mut spaces = VecDeque::new();
    let mut disk_size: usize = 0;

    for (i, c) in input.chars().enumerate() {
        // let block_length = c.to_digit(10).unwrap() as usize;
        let block_length = (c as u8 - b'0') as usize;
        if i % 2 == 0 {
            // is file
            let file_id = i / 2;
            files.push((disk_size, block_length, file_id));
        } else {
            spaces.push_back((disk_size, block_length));
            // is empty space
        }
        disk_size += block_length;
    }

    for i in (0..files.len()).rev() {
        // for j in 0..spaces.len() {
        for space in &mut spaces {
            if space.0 > files[i].0 {
                break;
            }
            match space.1.cmp(&files[i].1) {
                Ordering::Greater => {
                    // the first space is bigger, the whole file will go in it
                    files[i].0 = space.0;
                    space.0 += files[i].1;
                    space.1 -= files[i].1;
                }
                Ordering::Less => {
                    // the last file is bigger, break it up and fill space
                    continue;
                }
                Ordering::Equal => {
                    //  they are the same size, remove both
                    files[i].0 = space.0;
                    space.1 = 0;
                }
            }
        }
    }

    let checksum = calculate_checksum(files.into_iter());

    let end = bench_start.elapsed();
    eprintln!("{end:?}");
    Ok(checksum)
}

fn calculate_checksum<I: Iterator<Item = (usize, usize, usize)>>(files: I) -> usize {
    files
        .map(|(index, length, id)| {
            (index..index + length)
                .map(|index| index * id)
                .sum::<usize>()
        })
        .sum()
}
// fn calculate_checksum<I: Iterator<Item = (usize, usize, usize)>>(files: I) -> usize {
//     files.map(calculate_checksum_chunk).sum()
// }
//
// fn calculate_checksum_chunk(chunk: (usize, usize, usize)) -> usize {
//     ((chunk.1 * ((chunk.0 << 1) + chunk.1 - 1)) >> 1) * chunk.2
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [1928];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [2858];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
