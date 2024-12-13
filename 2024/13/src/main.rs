use float_cmp::{ApproxEq, F64Margin};
use itertools::Itertools;
use nalgebra::ComplexField;

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let mut total = 0;
    for machine in input.chunks(4) {
        let a = &machine[0];
        let b = &machine[1];
        let prize = &machine[2];
        let a: (usize, usize) = a
            .split('+')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<usize>()
                    .unwrap()
            })
            .tuples()
            .next()
            .unwrap();
        let b: (usize, usize) = b
            .split('+')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<usize>()
                    .unwrap()
            })
            .tuples()
            .next()
            .unwrap();
        let prize: (usize, usize) = prize
            .split('=')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<usize>()
                    .unwrap()
            })
            .tuples()
            .next()
            .unwrap();
        let mut min_cost: Option<usize> = None;
        let mut a_presses = 0;

        while a_presses <= 100 {
            let a_x_distance = a.0 * a_presses;
            let a_y_distance = a.1 * a_presses;
            if a_x_distance > prize.0 || a_y_distance > prize.1 {
                break;
            }
            let remaining_x = prize.0 - a_x_distance;
            let remaining_y = prize.1 - a_y_distance;
            if remaining_x % b.0 == 0 && remaining_y % b.1 == 0 {
                let b_presses = remaining_x / b.0;
                if remaining_y / b.1 == b_presses {
                    let cost = 3 * a_presses + b_presses;
                    if let Some(previous_min) = min_cost {
                        if cost < previous_min {
                            min_cost.replace(cost);
                        }
                    } else {
                        min_cost.replace(cost);
                    }
                }
            }
            a_presses += 1;
        }
        if let Some(cost) = min_cost {
            total += cost;
        }
    }
    Ok(total)
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let mut total = 0;
    for (idx, machine) in input.chunks(4).enumerate() {
        let a = &machine[0];
        let b = &machine[1];
        let prize = &machine[2];
        let a: (f64, f64) = a
            .split('+')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<f64>()
                    .unwrap()
            })
            .tuples()
            .next()
            .unwrap();
        let b: (f64, f64) = b
            .split('+')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<f64>()
                    .unwrap()
            })
            .tuples()
            .next()
            .unwrap();
        let prize: (f64, f64) = prize
            .split('=')
            .skip(1)
            .map(|piece| {
                piece
                    .strip_suffix(", Y")
                    .unwrap_or(piece)
                    .parse::<f64>()
                    .unwrap()
                    + 10000000000000f64
            })
            .tuples()
            .next()
            .unwrap();
        let (c1, c2) = prize;
        let (a1, a2) = a;
        let (b1, b2) = b;
        let dividend = c1 * b2 - b1 * c2;
        let divisor = a1 * b2 - b1 * a2;
        if divisor.approx_eq(0f64, F64Margin::default()) {
            continue;
        }
        let a_presses = dividend / divisor;
        if !a_presses.fract().approx_eq(0f64, F64Margin::default()) {
            continue;
        }
        let dividend = a1 * c2 - c1 * a2;
        let b_presses = dividend / divisor;
        if !b_presses.fract().approx_eq(0f64, F64Margin::default()) {
            continue;
        }
        total += (3 * a_presses.round() as i64) + b_presses.round() as i64;
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
    fn mmul_works() {
        let a = TwoTwo((1f64, 2f64), (3f64, 4f64));
        let b = TwoTwo((2f64, 0f64), (1f64, 2f64));
        let res = a * b;
        assert!(res.a().approx_eq(4f64, F64Margin::default()));
        assert!(res.b().approx_eq(4f64, F64Margin::default()));
        assert!(res.c().approx_eq(10f64, F64Margin::default()));
        assert!(res.d().approx_eq(8f64, F64Margin::default()));
        assert!(res.approx_eq(TwoTwo((4f64, 4f64), (10f64, 8f64)), F64Margin::default()));
    }

    #[test]
    fn mmul_works_pre_invert() {
        let a = TwoTwo((1f64, 2f64), (3f64, 4f64));
        let b = TwoTwo((-2f64, 1f64), (3f64 / 2f64, -0.5f64));
        let res = a * b;
        let ident = TwoTwo::ident();
        assert!(res.approx_eq(ident, F64Margin::default()));
    }

    #[test]
    fn minv_works() {
        let a = TwoTwo((1f64, 2f64), (3f64, 4f64));
        let res = a.invert().unwrap();
        let expected = TwoTwo((-2f64, 1f64), (3f64 / 2f64, -0.5f64));
        assert!(res.approx_eq(expected, F64Margin::default()));
    }

    #[test]
    fn mmultup_works() {
        let a = TwoTwo((1f64, 2f64), (3f64, 4f64));
        let res = a.mul_tup((5f64, 6f64));
        assert!(res.0.approx_eq(17f64, F64Margin::default()));
        assert!(res.1.approx_eq(39f64, F64Margin::default()));
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
