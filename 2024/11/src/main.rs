use std::collections::HashMap;

use tracing::info;
mod stone_table;

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(mut input: Vec<String>) -> anyhow::Result<usize> {
    let input = input.pop().unwrap();

    let stones: Vec<_> = input
        .split(" ")
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect();
    Ok(blinking_stones(25, &stones))
}

// 0
// 1
// 2024
// 20 24

fn blinking_stones(num_blinks: usize, initial_stones: &[usize]) -> usize {
    // let zeros_table = stone_table::table_of_zero_stones();
    let mut even_blinks: HashMap<usize, usize> = HashMap::with_capacity(10_000_000);
    let mut odd_blinks: HashMap<usize, usize> = HashMap::with_capacity(10_000_000);
    for &stone in initial_stones {
        *even_blinks.entry(stone).or_default() += 1;
    }
    for i in 0..num_blinks {
        info!("blink {i}");
        if i % 2 == 0 {
            for (stone, count) in &even_blinks {
                if *stone == 0 {
                    *odd_blinks.entry(1).or_default() += count;
                } else {
                    let num_digits = ((*stone as f64).log10() + 1f64).floor() as usize;
                    if num_digits % 2 == 0 {
                        let left_stone = stone / 10usize.pow((num_digits / 2) as u32);
                        let right_stone = stone % 10usize.pow((num_digits / 2) as u32);
                        *odd_blinks.entry(left_stone).or_default() += count;
                        *odd_blinks.entry(right_stone).or_default() += count;
                    } else {
                        *odd_blinks.entry(stone * 2024).or_default() += count;
                    }
                }
            }
            even_blinks.clear();
        } else {
            for (stone, count) in &odd_blinks {
                if *stone == 0 {
                    *even_blinks.entry(1).or_default() += count;
                } else {
                    let num_digits = ((*stone as f64).log10() + 1f64).floor() as usize;
                    if num_digits % 2 == 0 {
                        let left_stone = stone / 10usize.pow((num_digits / 2) as u32);
                        let right_stone = stone % 10usize.pow((num_digits / 2) as u32);
                        *even_blinks.entry(left_stone).or_default() += count;
                        *even_blinks.entry(right_stone).or_default() += count;
                    } else {
                        *even_blinks.entry(stone * 2024).or_default() += count;
                    }
                }
            }
            odd_blinks.clear();
        }
    }
    if num_blinks % 2 == 0 {
        even_blinks.values().sum()
    } else {
        odd_blinks.values().sum()
    }
}
// fn blinking_stones(num_blinks: usize, initial_stones: &[usize]) -> usize {
//     let zeros_table = stone_table::table_of_zero_stones();
//     let mut even_blinks = Vec::with_capacity(10_000_000);
//     let mut odd_blinks = Vec::with_capacity(10_000_000);
//     even_blinks.extend_from_slice(initial_stones);
//     let mut total = 0;
//     for i in 0..num_blinks {
//         info!("blink {i}");
//         if i % 2 == 0 {
//             for &stone in &even_blinks {
//                 if stone == 0 {
//                     let remaining_blinks = num_blinks - i - 1;
//                     if remaining_blinks < zeros_table.len() {
//                         total += zeros_table[remaining_blinks];
//                     } else {
//                         odd_blinks.push(1);
//                     }
//                 } else {
//                     let num_digits = ((stone as f64).log10() + 1f64).floor() as usize;
//                     if num_digits % 2 == 0 {
//                         let left_stone = stone / 10usize.pow((num_digits / 2) as u32);
//                         let right_stone = stone % 10usize.pow((num_digits / 2) as u32);
//                         odd_blinks.push(left_stone);
//                         odd_blinks.push(right_stone);
//                     } else {
//                         odd_blinks.push(stone * 2024);
//                     }
//                 }
//             }
//             even_blinks.truncate(0);
//             info!("after blink {i} : {} stones", odd_blinks.len());
//         } else {
//             for &stone in &odd_blinks {
//                 if stone == 0 {
//                     even_blinks.push(1);
//                 } else {
//                     let num_digits = ((stone as f64).log10() + 1f64).floor() as usize;
//                     if num_digits % 2 == 0 {
//                         let left_stone = stone / 10usize.pow((num_digits / 2) as u32);
//                         let right_stone = stone % 10usize.pow((num_digits / 2) as u32);
//                         even_blinks.push(left_stone);
//                         even_blinks.push(right_stone);
//                     } else {
//                         even_blinks.push(stone * 2024);
//                     }
//                 }
//             }
//             odd_blinks.truncate(0);
//             info!("after blink {i} : {} stones", even_blinks.len());
//         }
//     }
//     if num_blinks % 2 == 0 {
//         even_blinks.len() + total
//     } else {
//         odd_blinks.len() + total
//     }
// }

fn part2(mut input: Vec<String>) -> anyhow::Result<usize> {
    let input = input.pop().unwrap();

    let stones: Vec<_> = input
        .split(" ")
        .map(|stone| stone.parse::<usize>().unwrap())
        .collect();
    Ok(blinking_stones(75, &stones))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinking_stones() {
        assert_eq!(blinking_stones(6, &[125, 17]), 22);
    }

    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [55312];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }
    //
    // #[test]
    // fn test_part2() {
    //     todo!("Add test for part 2");
    //     // let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
    //     //     .iter()
    //     //     .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
    //     //     .collect();
    //     // let outputs = [42];
    //     // assert_eq!(inputs.len(), outputs.len());
    //     // for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
    //     //     assert_eq!(part2(input).unwrap(), output);
    //     // }
    // }
}
