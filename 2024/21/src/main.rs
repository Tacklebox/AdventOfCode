use common::grid::{
    coordinates::{Point, CARDINALS},
    Grid,
};
use itertools::Itertools;
use pathfinding::directed::astar::astar_bag_collect;

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<String> {
    let numeric_keypad = numeric_keypad();
    let direction_keypad = direction_keypad();
    // Point::from((2, 3))
    let code = input.first().unwrap();
    eprintln!("Looking for code {code}");
    let first_directions = solve_code(code, &numeric_keypad, Point::from((2, 3)));
    eprintln!(
        "found {} ways for the final robot to enter the code",
        first_directions.len()
    );
    let mut second_directions = Vec::new();
    for code in first_directions {
        second_directions.extend(solve_code(&code, &direction_keypad, Point::from((0, 0))));
    }
    let shortest_second = second_directions
        .iter()
        .map(|path| path.len())
        .min()
        .unwrap();
    second_directions = second_directions
        .into_iter()
        .filter(|path| path.len() == shortest_second)
        .collect_vec();
    eprintln!(
        "found {} ways for the second robot to tell the final robot to enter the code, that take {} steps",
        second_directions.len(),
        shortest_second,
    );
    let mut third_directions = Vec::new();
    for code in second_directions {
        third_directions.extend(solve_code(&code, &direction_keypad, Point::from((0, 0))));
    }
    let shortest_third = third_directions
        .iter()
        .map(|path| path.len())
        .min()
        .unwrap();
    third_directions = third_directions
        .into_iter()
        .filter(|path| path.len() == shortest_third)
        .collect_vec();
    eprintln!(
        "found {} ways for the third robot to tell the second robot to tell the final robot to enter the code of length {} steps",
        third_directions.len(),
        shortest_third
    );
    let mut human_directions = Vec::new();
    for code in third_directions {
        human_directions.extend(solve_code(&code, &direction_keypad, Point::from((0, 0))));
    }
    eprintln!(
        "found {} ways for you to tell the chain of robots to enter the code",
        human_directions.len()
    );
    Ok(human_directions
        .into_iter()
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap())
}

fn direction_keypad() -> Grid<Option<char>> {
    Grid::from_parts(
        vec![None, Some('^'), Some('A'), Some('<'), Some('v'), Some('>')],
        3,
        2,
    )
}

fn numeric_keypad() -> Grid<Option<char>> {
    Grid::from_parts(
        vec![
            Some('7'),
            Some('8'),
            Some('9'),
            Some('4'),
            Some('5'),
            Some('6'),
            Some('1'),
            Some('2'),
            Some('3'),
            None,
            Some('0'),
            Some('A'),
        ],
        3,
        4,
    )
}

fn solve_code(code: &str, keypad: &Grid<Option<char>>, start: Point) -> Vec<String> {
    // let numeric_keypad = numeric_keypad();
    let mut current_point = start;
    let mut possible_sequences: Vec<Vec<char>> = vec![Vec::new()];
    for c in code.chars() {
        let (paths_to_c, _) = astar_bag_collect(
            &current_point,
            |point| {
                let neighbours = CARDINALS
                    .iter()
                    .map(|d| (*point + *d, 1))
                    .filter(|(new_point, _)| keypad.get(*new_point).is_some())
                    .collect::<Vec<_>>();
                neighbours
            },
            |_| 0,
            |point| {
                keypad
                    .get(*point)
                    .unwrap()
                    .is_some_and(|button| button == c)
            },
        )
        .unwrap();
        let mut button_sequences = Vec::new();
        for path in paths_to_c {
            current_point = *path.last().unwrap();
            let mut button_sequence = Vec::new();
            for (&a, &b) in path.iter().tuple_windows() {
                match (a.0 - b.0, a.1 - b.1) {
                    (1, 0) => button_sequence.push('<'),
                    (-1, 0) => button_sequence.push('>'),
                    (0, 1) => button_sequence.push('^'),
                    (0, -1) => button_sequence.push('v'),
                    _ => unreachable!(),
                }
            }
            button_sequence.push('A');
            button_sequences.push(button_sequence);
        }
        let mut new_sequences: Vec<Vec<char>> = Vec::new();
        for prev in possible_sequences {
            for bs in &button_sequences {
                let mut new_sequence = prev.clone();
                new_sequence.extend(bs);
                new_sequences.push(new_sequence);
            }
        }
        possible_sequences = new_sequences;
    }
    possible_sequences
        .into_iter()
        .map(|vc| vc.iter().join(""))
        .collect()
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let _ = input;
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let inputs: Vec<String> = include_str!("../testcase_1.txt")
            .lines()
            .map(String::from)
            .collect();
        let outputs = [
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ];
        assert_eq!(inputs.len(), outputs.len());

        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(solve_code(&input), output);
        }
    }

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
