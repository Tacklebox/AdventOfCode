fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    let _ = input;
    todo!();
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let _ = input;
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        todo!("Add test for part 1");

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
