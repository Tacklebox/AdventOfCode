use std::collections::HashSet;

use common::{
    grid::{
        coordinates::{Point, CARDINALS},
        Grid,
    },
    search::SearchState,
};
use itertools::Itertools;
use pathfinding::directed::{bfs::bfs, count_paths::count_paths};

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

fn print_grid(grid: &Grid<char>) {
    for row in &grid.points().chunks(grid.width) {
        for point in row {
            print!("{}", grid.get(point).unwrap());
        }
        println!();
    }
}
fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    let (course, locations) = Grid::from_input(input, &['S', 'E']);
    // print_grid(&course);
    let start = locations.get(&'S').unwrap()[0];
    let end = locations.get(&'E').unwrap()[0];
    // let starting_state = RaceState {
    //     point: start,
    //     remaining_cheats: 2,
    // };
    let shortest_path = bfs(
        &start,
        |point| {
            let neighbours = CARDINALS
                .iter()
                .map(|c| *c + *point)
                .filter(|n| *course.get(*n).unwrap() != '#')
                .collect::<Vec<Point>>();
            neighbours
        },
        |point| *point == end,
    )
    .unwrap();
    let mut shortcuts = 0;
    for i in 0..shortest_path.len() {
        for j in i + 102..shortest_path.len() {
            if shortest_path[i].dist(shortest_path[j]) == 2 {
                if shortest_path[i].0 == shortest_path[j].0
                    || shortest_path[i].1 == shortest_path[j].1
                {
                    shortcuts += 1;
                } else {
                    shortcuts += 2;
                }
            }
        }
    }

    Ok(shortcuts)
}

fn part2(input: Vec<String>) -> anyhow::Result<i64> {
    let (course, locations) = Grid::from_input(input, &['S', 'E']);
    // print_grid(&course);
    let start = locations.get(&'S').unwrap()[0];
    let end = locations.get(&'E').unwrap()[0];
    // let starting_state = RaceState {
    //     point: start,
    //     remaining_cheats: 2,
    // };
    let shortest_path = bfs(
        &start,
        |point| {
            let neighbours = CARDINALS
                .iter()
                .map(|c| *c + *point)
                .filter(|n| *course.get(*n).unwrap() != '#')
                .collect::<Vec<Point>>();
            neighbours
        },
        |point| *point == end,
    )
    .unwrap();
    let mut shortcuts = 0;
    for i in 0..shortest_path.len() {
        for j in i + 102..shortest_path.len() {
            let dist = shortest_path[i].dist(shortest_path[j]);
            if dist <= 20 && (j - i) - dist >= 100 {
                shortcuts += 1;
            }
        }
    }

    Ok(shortcuts)
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
        let outputs = [44];
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
        let outputs = [285];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }
}
