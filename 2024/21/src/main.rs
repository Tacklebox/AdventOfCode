use std::collections::HashMap;

use common::grid::{
    coordinates::{Point, CARDINALS},
    Grid,
};
use itertools::Itertools;
use pathfinding::directed::astar::astar_bag_collect;

type Keypad = HashMap<char, Point>;
fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<String> {
    todo!()
}

fn direction_keypad() -> Keypad {
    HashMap::from([
        ('^', Point(1, 0)),
        ('A', Point(2, 0)),
        ('<', Point(0, 1)),
        ('v', Point(1, 1)),
        ('>', Point(2, 1)),
    ])
}

fn numeric_keypad() -> Keypad {
    HashMap::from([
        ('7', Point(0, 0)),
        ('8', Point(1, 0)),
        ('9', Point(2, 0)),
        ('4', Point(0, 1)),
        ('5', Point(1, 1)),
        ('4', Point(2, 1)),
        ('1', Point(0, 2)),
        ('2', Point(1, 2)),
        ('3', Point(2, 2)),
        ('0', Point(1, 3)),
        ('A', Point(2, 3)),
    ])
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {}

fn solve_directions(directions: &str, keypad: &Keypad, depth: usize) -> String {
    if depth == 0 {
        return directions.to_owned();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

        // let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
        //     .iter()
        //     .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
        //     .collect();
        // let outputs = [42];
        // assert_eq!(inputs.len(), outputs.len());
        // for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
        //     assert_eq!(part1(input).unwrap(), output);
        // }
    }

    #[test]
    fn test_part2() {
        todo!("Add test for part 2");
        // let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
        //     .iter()
        //     .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
        //     .collect();
        // let outputs = [42];
        // assert_eq!(inputs.len(), outputs.len());
        // for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
        //     assert_eq!(part2(input).unwrap(), output);
        // }
    }
}
