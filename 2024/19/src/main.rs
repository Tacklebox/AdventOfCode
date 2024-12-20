use std::{collections::HashMap, time::Instant};

#[derive(Debug, Default)]
struct StringTrie {
    trie: radix_trie::Trie<String, ()>,
    largest_element: usize,
}

impl StringTrie {
    fn insert_str(&mut self, s: &str) {
        self.largest_element = self.largest_element.max(s.len());
        self.trie.insert(s.to_string(), ());
    }

    fn contains(&self, s: &str) -> bool {
        if s.len() > self.largest_element {
            return false;
        }
        self.trie.get(s).is_some()
    }
}

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn search_trie(design: &str, trie: &StringTrie) -> bool {
    fn recurse(design: &str, start: usize, trie: &StringTrie) -> bool {
        if start == design.len() {
            return true;
        }
        let suffix = &design[start..];

        for i in (1..=suffix.len()).rev() {
            let possible_prefix = &suffix[..i];
            if trie.contains(possible_prefix) && recurse(design, start + i, trie) {
                return true;
            }
        }
        false
    }

    recurse(design, 0, trie)
}

fn search_trie_count(design: &str, trie: &StringTrie) -> usize {
    let mut memo = HashMap::new();
    fn recurse(
        design: &str,
        start: usize,
        trie: &StringTrie,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if start == design.len() {
            return 1;
        }
        let suffix = &design[start..];
        if let Some(&combinations) = memo.get(suffix) {
            return combinations;
        }

        let mut total = 0;
        for i in (1..=suffix.len()).rev() {
            let possible_prefix = &suffix[..i];
            if trie.contains(possible_prefix) {
                total += recurse(design, start + i, trie, memo);
            }
        }
        memo.insert(suffix.to_owned(), total);
        total
    }

    recurse(design, 0, trie, &mut memo)
}

fn parse_input(input: &[String]) -> (Vec<String>, Vec<String>) {
    let mut input_iter = input.iter();
    let available_towels = input_iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let desired_patterns = input_iter.skip(1).map(|s| s.to_string()).collect();
    (available_towels, desired_patterns)
}

fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    let (towels, designs) = parse_input(&input);
    let mut trie = StringTrie::default();
    for towel in &towels {
        trie.insert_str(towel);
    }

    let mut possible_designs = 0;
    for design in designs {
        if search_trie(&design, &trie) {
            possible_designs += 1;
        }
    }

    Ok(possible_designs)
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    // let (towels, mut designs) = parse_input(&input);
    let (towels, designs) = parse_input(&input);
    let start = Instant::now();
    let mut trie = StringTrie::default();
    for towel in &towels {
        trie.insert_str(towel);
    }

    // let n_designs = designs.len();
    // designs.retain(|design| search_trie(design, &trie));
    let mut total = 0;
    // let n_pdesigns = designs.len();
    // eprintln!("Of the original {n_designs}, {n_pdesigns} are possible");
    for (i, design) in designs.into_iter().enumerate() {
        // eprintln!("calculating number of ways to make design #{i}");
        total += search_trie_count(&design, &trie);
    }
    let elapsed = start.elapsed();
    eprintln!("elapsed time in millis {}", elapsed.as_millis());

    Ok(total)
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
        let outputs = [6];
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
        let outputs = [16];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
