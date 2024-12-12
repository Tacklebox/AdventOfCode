#![feature(portable_simd)]
#![feature(stdarch_x86_avx512)]
#![feature(test)]

use std::io::Read;
use std::simd::{Mask, Simd};
use std::{cmp::Ordering, collections::VecDeque, time::Instant};
extern crate test;

use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input)?;
    let mut input = input.into_bytes();
    *input.last_mut().unwrap() = b'0';
    // println!("First 10 u8s in input before {:?}", &input[0..10]);

    let mut avx_input = input.clone();
    let start = Instant::now();
    unsafe {
        avx512_parse(&mut avx_input);
    }
    let duration = start.elapsed();
    println!(
        "Converting the input to u8 with avx took {} nanoseconds",
        duration.as_nanos()
    );

    let mut simd_input = input.clone();
    let start = Instant::now();
    simd_parse(&mut simd_input);
    let duration = start.elapsed();
    println!(
        "Converting the input to u8 with simd took {} nanoseconds",
        duration.as_nanos()
    );

    let mut iter_input = input.clone();
    let start = Instant::now();
    iterator_parse(&mut iter_input);
    let duration = start.elapsed();
    println!(
        "Converting the input to u8 with iterators took {} nanoseconds",
        duration.as_nanos()
    );

    // let mut par_iter_input = input.clone();
    // let start = Instant::now();
    // par_iter_parse(&mut par_iter_input);
    // let duration = start.elapsed();
    // println!(
    //     "Converting the input to u8 with multi-threaded iterators took {} nanoseconds",
    //     duration.as_nanos()
    // );

    //
    // println!("First 10 u8s in input after {:?}", &input[0..10]);

    // println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn iterator_parse(input: &mut [u8]) {
    for c in input.iter_mut() {
        *c -= b'0';
    }
}

fn par_iter_parse(input: &mut [u8]) {
    input.par_iter_mut().for_each(|c| {
        *c -= b'0';
    });
}

pub unsafe fn avx512_parse(input: &mut [u8]) {
    use std::arch::x86_64::*;
    let chunk_size = 64; // AVX-512 processes 64 bytes in parallel
    let chunks = input.chunks_exact_mut(chunk_size);

    for chunk in chunks {
        // Load 64 bytes into an AVX-512 register
        let mut avx_data = std::arch::x86_64::_mm512_loadu_si512(chunk.as_ptr() as *const i32);

        // std::arch::x86_64::_mm512_loadu_si512(mem_addr);
        // Subtract '0' (48) from each byte
        let avx_zero = _mm512_set1_epi8(b'0' as i8);
        avx_data = _mm512_sub_epi8(avx_data, avx_zero);

        // Store the result back into the slice
        _mm512_storeu_si512(chunk.as_mut_ptr() as *mut i32, avx_data);
    }
}

pub unsafe fn avx2_parse(input: &mut [u8]) {
    use std::arch::x86_64::*;
    let chunk_size = 32; // AVX2 processes 32 bytes in parallel
    let chunks = input.chunks_exact_mut(chunk_size);

    for chunk in chunks {
        // Load 32 bytes into an AVX2 register
        let avx_data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);

        // Subtract '0' (48) from each byte
        let avx_zero = _mm256_set1_epi8(b'0' as i8);
        let result = _mm256_sub_epi8(avx_data, avx_zero);

        // Store the result back into the slice
        _mm256_storeu_si256(chunk.as_mut_ptr() as *mut __m256i, result);
    }
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

fn simd_parse(input: &mut [u8]) {
    let chunk_size = 32;
    let chunks = input.chunks_exact_mut(chunk_size);

    for chunk in chunks {
        let simd_chunk: Simd<u8, 32> = Simd::from_slice(chunk);

        let result = simd_chunk - Simd::splat(b'0');

        unsafe {
            result.store_select_unchecked(chunk, Mask::splat(true));
        }
    }
}

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

    #[bench]
    fn bench_simd_parsing(b: &mut test::Bencher) {
        let mut input = std::fs::read("random_digits.txt").unwrap();
        b.iter(|| {
            simd_parse(&mut input);
        })
    }

    #[bench]
    fn bench_iter_parsing(b: &mut test::Bencher) {
        let mut input = std::fs::read("random_digits.txt").unwrap();
        b.iter(|| {
            iterator_parse(&mut input);
        })
    }

    #[bench]
    fn bench_par_iter_parsing(b: &mut test::Bencher) {
        let mut input = std::fs::read("random_digits.txt").unwrap();
        b.iter(|| {
            par_iter_parse(&mut input);
        })
    }

    // #[bench]
    // fn bench_avx_parsing(b: &mut test::Bencher) {
    //     let mut input = std::fs::read("random_digits.txt").unwrap();
    //     b.iter(|| unsafe {
    //         avx512_parse(&mut input);
    //     })
    // }

    #[bench]
    fn bench_avx2_parsing(b: &mut test::Bencher) {
        let mut input = std::fs::read("random_digits.txt").unwrap();
        b.iter(|| unsafe {
            avx2_parse(&mut input);
        })
    }
}
