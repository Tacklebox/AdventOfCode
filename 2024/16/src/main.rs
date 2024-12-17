use std::collections::{BinaryHeap, HashSet};

use anyhow::bail;
use common::grid::coordinates::{Point, EAST, NORTH, SOUTH, WEST};
use common::grid::Grid;
use common::search::graph_search;
use pathfinding::directed::astar::astar_bag;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MazeItem {
    Hedge,
    Goal,
}

fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}

type Maze = Grid<Option<MazeItem>>;
fn parse_maze(input: Vec<String>) -> (Maze, Point) {
    let mut starting_point = None;
    let height = input.len();
    let mut storage = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => storage.push(Some(MazeItem::Hedge)),
                '.' => storage.push(None),
                'E' => storage.push(Some(MazeItem::Goal)),
                'S' => {
                    storage.push(None);
                    starting_point.replace((x, y).into());
                }
                _ => unreachable!("Invalid char in input"),
            }
        }
    }
    let width = storage.len() / height;
    (
        Grid::from_parts(storage, width, height),
        starting_point.unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct MazeSearchState {
    position: Point,
    direction: Point,
}

fn part1(input: Vec<String>) -> anyhow::Result<i64> {
    let (maze, start) = parse_maze(input);
    let initial_state = MazeSearchState {
        position: start,
        direction: EAST,
    };
    if let Some(result) = graph_search(
        initial_state,
        |state| {
            maze.get(state.position)
                .unwrap()
                .is_some_and(|el| el == MazeItem::Goal)
        },
        |prev_state| {
            let mut next_states: Vec<(MazeSearchState, i64)> = Vec::new();
            if !matches!(
                maze.get(prev_state.position + prev_state.direction)
                    .unwrap(),
                Some(MazeItem::Hedge)
            ) {
                let new_pos = prev_state.position + prev_state.direction;
                next_states.push((
                    MazeSearchState {
                        position: new_pos,
                        direction: prev_state.direction,
                    },
                    1,
                ));
            }
            if prev_state.direction == NORTH || prev_state.direction == SOUTH {
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: EAST,
                    },
                    1000,
                ));
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: WEST,
                    },
                    1000,
                ));
            } else {
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: NORTH,
                    },
                    1000,
                ));
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: SOUTH,
                    },
                    1000,
                ));
            }
            next_states
        },
        &mut BinaryHeap::new(),
    ) {
        return Ok(result.cost);
    }
    bail!("No path to goal found");
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    let (maze, start) = parse_maze(input);
    let initial_state = MazeSearchState {
        position: start,
        direction: EAST,
    };
    if let Some((best_paths, _)) = astar_bag(
        &initial_state,
        |prev_state| {
            let mut next_states: Vec<(MazeSearchState, i64)> = Vec::new();
            if !matches!(
                maze.get(prev_state.position + prev_state.direction)
                    .unwrap(),
                Some(MazeItem::Hedge)
            ) {
                let new_pos = prev_state.position + prev_state.direction;
                next_states.push((
                    MazeSearchState {
                        position: new_pos,
                        direction: prev_state.direction,
                    },
                    1,
                ));
            }
            if prev_state.direction == NORTH || prev_state.direction == SOUTH {
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: EAST,
                    },
                    1000,
                ));
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: WEST,
                    },
                    1000,
                ));
            } else {
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: NORTH,
                    },
                    1000,
                ));
                next_states.push((
                    MazeSearchState {
                        position: prev_state.position,
                        direction: SOUTH,
                    },
                    1000,
                ));
            }
            next_states
        },
        |_| 0,
        |state| {
            maze.get(state.position)
                .unwrap()
                .is_some_and(|el| el == MazeItem::Goal)
        },
    ) {
        let mut best_seats_in_the_maze = HashSet::new();
        for path in best_paths {
            for point in path {
                best_seats_in_the_maze.insert(point.position);
            }
        }
        return Ok(best_seats_in_the_maze.len());
    }
    bail!("No path to goal found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let inputs: Vec<Vec<String>> = [
            include_str!("../testcase_1.txt"),
            include_str!("../testcase_2.txt"),
        ]
        .iter()
        .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
        .collect();
        let outputs = [7036, 11048];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [
            include_str!("../testcase_1.txt"),
            include_str!("../testcase_2.txt"),
        ]
        .iter()
        .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
        .collect();
        let outputs = [45, 64];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
