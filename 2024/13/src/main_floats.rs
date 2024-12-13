use std::ops::Mul;

use float_cmp::{ApproxEq, F64Margin};
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct TwoTwo((f64, f64), (f64, f64));

impl TwoTwo {
    fn a(&self) -> f64 {
        self.0 .0
    }
    fn b(&self) -> f64 {
        self.0 .1
    }
    fn c(&self) -> f64 {
        self.1 .0
    }
    fn d(&self) -> f64 {
        self.1 .1
    }

    fn ident() -> Self {
        Self((1f64, 0f64), (0f64, 1f64))
    }
    fn invert(&self) -> Option<Self> {
        let margin = F64Margin::default();
        let det = (self.a() * self.d()) - (self.b() * self.c());
        if det.approx_eq(0f64, margin) {
            return None;
        }

        let new_a = self.d() / det;
        let new_b = (-self.b()) / det;
        let new_c = (-self.c()) / det;
        let new_d = self.a() / det;
        let inv = Self((new_a, new_b), (new_c, new_d));

        let ident = *self * inv;
        if ident.approx_eq(Self::ident(), margin) {
            Some(inv)
        } else {
            None
        }
    }

    fn mul_tup(&self, tup: (f64, f64)) -> (f64, f64) {
        (
            (self.a() * tup.0 + self.b() * tup.1),
            (self.c() * tup.0 + self.d() * tup.1),
        )
    }
}

impl ApproxEq for TwoTwo {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();
        self.a().approx_eq(other.a(), margin)
            && self.b().approx_eq(other.b(), margin)
            && self.c().approx_eq(other.c(), margin)
            && self.d().approx_eq(other.d(), margin)
    }
    // add code here
}
impl Mul for TwoTwo {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = (self.a() * rhs.a()) + (self.b() * rhs.c());
        let b = (self.a() * rhs.b()) + (self.b() * rhs.d());
        let c = (self.c() * rhs.a()) + (self.d() * rhs.c());
        let d = (self.c() * rhs.b()) + (self.d() * rhs.d());
        Self((a, b), (c, d))
    }
    // add code here
}

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

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
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
        let system = TwoTwo(a, b);
        if let Some(inverted_system) = system.invert() {
            println!("inverse found for machine {idx}: {inverted_system:?}");
            let (a_presses, b_presses) = inverted_system.mul_tup(prize);
            println!("inverse multiplied by prize results in {a_presses} a and {b_presses} b");
            if a_presses.approx_eq(a_presses.round(), F64Margin::default())
                && b_presses.approx_eq(b_presses.round(), F64Margin::default())
                && a_presses.round() >= 0f64
                && b_presses.round() >= 0f64
            {
                let cost = (3 * a_presses.round() as usize) + b_presses.round() as usize;
                println!("Machine {idx} cost {cost} tokens");
                total += cost;
            }
        } else {
            println!("No inverse found for machine {idx}");
        }
    }
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
