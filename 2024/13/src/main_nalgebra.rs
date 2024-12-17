use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn parse_system(a: &str, b: &str, prize: &str) -> (Vector2<f64>, Vector2<f64>, Vector2<f64>) {
    let a = a
        .split('+')
        .skip(1)
        .map(|piece| {
            piece
                .strip_suffix(", Y")
                .unwrap_or(piece)
                .parse::<f64>()
                .unwrap()
        })
        .next_tuple::<(f64, f64)>()
        .map(|t| Vector2::new(t.0, t.1))
        .unwrap();
    let b = b
        .split('+')
        .skip(1)
        .map(|piece| {
            piece
                .strip_suffix(", Y")
                .unwrap_or(piece)
                .parse::<f64>()
                .unwrap()
        })
        .next_tuple::<(f64, f64)>()
        .map(|t| Vector2::new(t.0, t.1))
        .unwrap();
    let prize = prize
        .split('=')
        .skip(1)
        .map(|piece| {
            piece
                .strip_suffix(", Y")
                .unwrap_or(piece)
                .parse::<f64>()
                .unwrap()
        })
        .next_tuple::<(f64, f64)>()
        .map(|t| Vector2::new(t.0, t.1))
        .unwrap();
    (a, b, prize)
}

fn solve_machine(
    (button_a, button_b, prize): (Vector2<f64>, Vector2<f64>, Vector2<f64>),
) -> Option<(usize, usize)> {
    let a = Matrix2::from_rows(&[button_a.transpose(), button_b.transpose()]);
    let b = prize;
    println!("Finding solution for {a}[x y] = {b}");
    if let Some(solution) = a.lu().solve(&b) {
        println!("Solution: x = {}, y = {}", solution[0], solution[1]);
        if solution[0].fract().abs() <= 4f64 * f64::EPSILON
            && solution[1].fract().abs() <= 4f64 * f64::EPSILON
        {
            return Some((solution[0].round() as usize, solution[1].round() as usize));
        }
    }
    None
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let mut total = 0;
    for machine in input.chunks(4) {
        let system = parse_system(&machine[0], &machine[1], &machine[2]);
        if let Some(presses) = solve_machine(system) {
            if presses.0 <= 100 && presses.1 <= 100 {
                total += 3 * presses.0 + presses.1;
            }
        }
    }
    Ok(total)
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    let mut total = 0;
    for machine in input.chunks(4) {
        let system = parse_system(&machine[0], &machine[1], &machine[2]);
        if let Some(presses) = solve_machine(system) {
            total += 3 * presses.0 + presses.1;
        }
    }
    Ok(total)
}

// In rust Solves the system of linear equations:
// a.0 * a_count + b.0 * b_count = p.0
// a.1 * a_count + b.1 * b_count = p.1
// Returns Some((a_count, b_count)) if a solution exists, otherwise None.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_1.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [480];
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
        let outputs = [875318608908];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
