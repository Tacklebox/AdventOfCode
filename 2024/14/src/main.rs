use std::collections::HashSet;

use common::grid::coordinates::Point;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    let bots = parse_input(input);

    let width = 101;
    let height = 103;
    let resulting_positions = simulate_robots(100, height, width, &bots);
    let mut quadrants = (0, 0, 0, 0);
    let middle_column = (width / 2) as i64;
    let middle_row = (height / 2) as i64;
    for Point(x, y) in resulting_positions {
        if x < middle_column && y < middle_row {
            quadrants.0 += 1;
        } else if x > middle_column && y < middle_row {
            quadrants.1 += 1;
        } else if x > middle_column && y > middle_row {
            quadrants.2 += 1;
        } else if x < middle_column && y > middle_row {
            quadrants.3 += 1;
        }
    }
    Ok(quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3)
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let bots = parse_input(input);

    let width = 101;
    let height = 103;

    for i in 1..(101 * 103) {
        // for i in 1..100000000000 {
        let resulting_positions = simulate_robots_set(i, height, width, &bots);
        for y in 0..height {
            for x in 0..width {
                if resulting_positions.contains(&(x, y)) {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
    Ok(0)
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn from_str(serialized_robot: &str) -> Self {
        let (position, velocity): ((i64, i64), (i64, i64)) = serialized_robot
            .split(' ')
            .map(|chunk| {
                chunk
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .next_tuple::<(i64, i64)>()
                    .unwrap()
            })
            .next_tuple()
            .unwrap();
        Self {
            position: position.into(),
            velocity: velocity.into(),
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Robot> {
    input.iter().map(|line| Robot::from_str(line)).collect()
}

// x starts at 2
// x velocity is + 2
// simulating 5s
// width is 11
// ends at x 1
// total distance travelled is 2 * 5 = 10
// landing spot is 2 + (10 % 11) == 1

// y starts at 4
// y velocity is -3
// simulating 5s
// height is 7
// ends at y 3
// total distance travelled is -3 * 5 = -15
// landing spot is 4 + (-15 % 7) == 3

fn simulate_robots(n_seconds: usize, height: usize, width: usize, robots: &[Robot]) -> Vec<Point> {
    let mut results = Vec::new();
    for &bot in robots {
        let x = (bot.position.0 + (n_seconds as i64 * bot.velocity.0)).rem_euclid(width as i64);
        let y = (bot.position.1 + (n_seconds as i64 * bot.velocity.1)).rem_euclid(height as i64);
        results.push(Point::from((x, y)));
    }
    results
}

fn simulate_robots_set(
    n_seconds: usize,
    height: usize,
    width: usize,
    robots: &[Robot],
) -> HashSet<(usize, usize)> {
    let mut results = HashSet::new();
    for &bot in robots {
        let x = (bot.position.0 + (n_seconds as i64 * bot.velocity.0)).rem_euclid(width as i64);
        let y = (bot.position.1 + (n_seconds as i64 * bot.velocity.1)).rem_euclid(height as i64);
        results.insert((x as usize, y as usize));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bot() {
        let robots = &[Robot {
            position: (2i64, 4i64).into(),
            velocity: (2i64, -3i64).into(),
        }];
        let result = simulate_robots(5, 7, 11, robots);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Point::from((1i64, 3i64)));
    }
    #[test]
    fn test_part1_mini() {
        let mut inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let input = inputs.pop().unwrap();
        let bots = parse_input(input);
        let width = 11;
        let height = 7;
        let resulting_positions = simulate_robots(100, height, width, &bots);
        let mut quadrants = (0, 0, 0, 0);
        let middle_column = (width / 2) as i64;
        let middle_row = (height / 2) as i64;
        for Point(x, y) in resulting_positions {
            if x < middle_column && y < middle_row {
                quadrants.0 += 1;
            } else if x > middle_column && y < middle_row {
                quadrants.1 += 1;
            } else if x > middle_column && y > middle_row {
                quadrants.2 += 1;
            } else if x < middle_column && y > middle_row {
                quadrants.3 += 1;
            }
        }

        println!("0: {}", quadrants.0);
        println!("0: {}", quadrants.1);
        println!("0: {}", quadrants.2);
        println!("0: {}", quadrants.3);
        assert_eq!(quadrants.0, 1);
        assert_eq!(quadrants.1, 3);
        assert_eq!(quadrants.2, 1);
        assert_eq!(quadrants.3, 4);
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
