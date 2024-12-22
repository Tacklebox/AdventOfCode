use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct SecretSequence {
    current_value: i64,
}
impl SecretSequence {
    fn new(current_value: i64) -> Self {
        Self { current_value }
    }
}

impl Iterator for SecretSequence {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.current_value;
        self.current_value = ((64 * self.current_value) ^ self.current_value) % 16777216;
        self.current_value = ((self.current_value / 32) ^ self.current_value) % 16777216;
        self.current_value = ((2048 * self.current_value) ^ self.current_value) % 16777216;
        Some(n)
    }
}

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    Ok(input
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .map(SecretSequence::new)
        .map(|mut sequence| sequence.nth(2000).unwrap())
        .sum())
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let monkeys = input
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect_vec();
    let mut sequence_bananas = HashMap::new();
    for (monkey_number, monkey) in monkeys.into_iter().enumerate() {
        let sequence = SecretSequence::new(monkey);
        sequence
            .map(|s| (s, s % 10))
            .take(2000)
            .tuple_windows()
            .map(|(n, np)| (n, np, np.1 - n.1))
            .tuple_windows()
            .for_each(|(d1, d2, d3, d4)| {
                let entry = sequence_bananas
                    .entry((d1.2, d2.2, d3.2, d4.2))
                    .or_insert((HashSet::new(), 0));
                if entry.0.insert(monkey_number) {
                    entry.1 += d4.1 .1;
                }
            });
    }
    let eg_seq = sequence_bananas.get(&(-2, 1, -1, 3)).unwrap();
    // eprintln!(
    //     "the example sequence was (-2, 1, -1, 3), with a price of {}",
    //     eg_seq.1
    // );
    let best_seq = sequence_bananas.into_iter().max_by_key(|s| s.1 .1).unwrap();
    // eprintln!(
    //     "the best sequence was {:?}, with a price of {}",
    //     best_seq.0, best_seq.1 .1
    // );
    Ok(best_seq.1 .1)
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
        let outputs = [37327623];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_2.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [23];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
