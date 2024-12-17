use std::collections::HashMap;

use common::grid::{
    coordinates::{Point, EAST, NORTH, SOUTH, WEST},
    Grid,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WarehouseObject {
    Movable,
    Immovable,
    MovableLeft,
    MovableRight,
}
fn main() -> anyhow::Result<()> {
    println!("{}", common::advent(part1, part2)?);
    Ok(())
}
fn print_warehouse(warehouse: &Grid<Option<WarehouseObject>>, robot_position: Point) {
    for point in warehouse.points() {
        if point == robot_position {
            print!("@");
        } else {
            match warehouse.get(point).unwrap() {
                Some(WarehouseObject::Movable) => print!("O"),
                Some(WarehouseObject::Immovable) => print!("#"),
                Some(WarehouseObject::MovableLeft) => print!("["),
                Some(WarehouseObject::MovableRight) => print!("]"),
                None => print!("."),
            }
        }
        if point.0 as usize == warehouse.width - 1 {
            println!();
        }
    }
}

// ..[]@
fn score_gps2(grid: Grid<Option<WarehouseObject>>) -> usize {
    let mut total = 0;
    for point in grid.points() {
        if let Some(WarehouseObject::MovableLeft) = grid.get(point).unwrap() {
            total += (100 * point.1 as usize) + point.0 as usize;
        }
    }
    total
}

fn score_gps(grid: Grid<Option<WarehouseObject>>) -> usize {
    let mut total = 0;
    for point in grid.points() {
        if let Some(WarehouseObject::Movable) = grid.get(point).unwrap() {
            total += (100 * point.1 as usize) + point.0 as usize;
        }
    }
    total
}

fn parse_program(input: &[String]) -> Vec<Point> {
    let mut program = Vec::new();
    for line in input {
        for c in line.chars() {
            match c {
                '^' => program.push(NORTH),
                'v' => program.push(SOUTH),
                '<' => program.push(WEST),
                '>' => program.push(EAST),
                _ => unreachable!(),
            }
        }
    }
    program
}

fn simulate_warehouse(
    warehouse: Grid<Option<WarehouseObject>>,
    program: Vec<Point>,
    robot_position: Point,
) -> Grid<Option<WarehouseObject>> {
    let mut ww = warehouse.clone();
    let mut robot_position = robot_position;
    for instruction in program {
        let mut pos = robot_position + instruction;
        if ww.get(pos).unwrap().is_none() {
            robot_position = pos;
            continue;
        }
        // @OO#.
        while let Some(WarehouseObject::Movable) = ww.get(pos).unwrap() {
            pos = pos + instruction;
        }

        if ww.get(pos).unwrap().is_none() {
            //robot_position + instruction;
            ww.set(pos, Some(WarehouseObject::Movable));
            robot_position = robot_position + instruction;
            ww.set(robot_position, None);
            continue;
        }
    }
    ww
}

// #...@
// ##......@.
fn parse_map2(input: &[String]) -> (Grid<Option<WarehouseObject>>, Option<Point>) {
    let mut robot_position: Option<Point> = None;
    let height = input.len();
    let mut storage = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    robot_position.replace((2 * x as i64, y as i64).into());
                    storage.push(None);
                    storage.push(None);
                }
                '.' => storage.extend_from_slice(&[None, None]),
                '#' => storage.extend_from_slice(&[
                    Some(WarehouseObject::Immovable),
                    Some(WarehouseObject::Immovable),
                ]),
                'O' => storage.extend_from_slice(&[
                    Some(WarehouseObject::MovableLeft),
                    Some(WarehouseObject::MovableRight),
                ]),
                _ => unreachable!(),
            }
        }
    }
    let width = storage.len() / height;
    (Grid::from_parts(storage, width, height), robot_position)
}

fn parse_map(input: &[String]) -> (Grid<Option<WarehouseObject>>, Option<Point>) {
    let mut robot_position: Option<Point> = None;
    let height = input.len();
    let mut storage = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    robot_position.replace((x as i64, y as i64).into());
                    storage.push(None);
                }
                '.' => storage.push(None),
                '#' => storage.push(Some(WarehouseObject::Immovable)),
                'O' => storage.push(Some(WarehouseObject::Movable)),
                _ => unreachable!(),
            }
        }
    }
    let width = storage.len() / height;
    (Grid::from_parts(storage, width, height), robot_position)
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let input_split = input
        .iter()
        .by_ref()
        .position(|line| line.is_empty())
        .unwrap();
    let (warehouse, robot_position) = parse_map(&input[..input_split]);
    let program = parse_program(&input[input_split + 1..]);
    let final_positions = simulate_warehouse(warehouse, program, robot_position.unwrap());
    let score = score_gps(final_positions);
    Ok(score)
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    let input_split = input
        .iter()
        .by_ref()
        .position(|line| line.is_empty())
        .unwrap();
    let (warehouse, robot_position) = parse_map2(&input[..input_split]);
    let program = parse_program(&input[input_split + 1..]);
    // println!("program: {program:?}");
    // println!("initial state:");
    // print_warehouse(&warehouse, robot_position.unwrap());
    let final_positions = simulate_warehouse2(warehouse, program, robot_position.unwrap());
    // print_warehouse(&final_positions, (0i64, 0i64).into());
    // Ok(0)
    let score = score_gps2(final_positions);
    Ok(score)
}

fn simulate_warehouse2(
    warehouse: Grid<Option<WarehouseObject>>,
    program: Vec<Point>,
    robot_position: Point,
) -> Grid<Option<WarehouseObject>> {
    let mut ww = warehouse.clone();
    let mut robot_position = robot_position;
    'outer: for instruction in program {
        // println!("before instruction {instruction:?}");
        // print_warehouse(&ww, robot_position);
        let in_front = robot_position + instruction;
        let mut target_pos = in_front;
        if instruction == NORTH || instruction == SOUTH {
            if ww.get(target_pos).unwrap().is_none() {
                robot_position = target_pos;
                continue;
            }
            let mut pending_moves: HashMap<Point, Option<WarehouseObject>> =
                HashMap::from_iter([(target_pos, None)]);
            let mut target_positions = vec![target_pos];
            while !target_positions.is_empty() {
                let mut next_targets: Vec<Point> = Vec::new();
                for &pos in &target_positions {
                    let item = ww.get(pos).unwrap();
                    if item.is_none() {
                        continue;
                    }
                    if let Some(WarehouseObject::MovableLeft) = item {
                        // if [] add pendign moves to . both of them
                        //    ^
                        pending_moves.entry(pos).or_insert(None);
                        pending_moves.entry(pos + EAST).or_insert(None);

                        // add pending move to add new [] past them
                        pending_moves.insert(pos + instruction, Some(WarehouseObject::MovableLeft));
                        pending_moves.insert(
                            pos + EAST + instruction,
                            Some(WarehouseObject::MovableRight),
                        );
                        // add search targets for space past them
                        next_targets.push(pos + instruction);
                        next_targets.push(pos + EAST + instruction);
                    }
                    if let Some(WarehouseObject::MovableRight) = item {
                        // if [] add pendign moves to . both of them
                        //     ^
                        pending_moves.entry(pos).or_insert(None);
                        pending_moves.entry(pos + WEST).or_insert(None);

                        // add pending move to add new [] past them
                        pending_moves
                            .insert(pos + instruction, Some(WarehouseObject::MovableRight));
                        pending_moves
                            .insert(pos + WEST + instruction, Some(WarehouseObject::MovableLeft));
                        // add search targets for space past them
                        next_targets.push(pos + instruction);
                        next_targets.push(pos + WEST + instruction);
                    }
                    if let Some(WarehouseObject::Immovable) = item {
                        continue 'outer;
                    }
                }
                target_positions = next_targets;
            }
            for (position, warehouse_thing) in pending_moves.into_iter() {
                ww.set(position, warehouse_thing);
            }
            robot_position = in_front;
        } else {
            if ww.get(target_pos).unwrap().is_none() {
                robot_position = target_pos;
                continue;
            }

            while let Some(WarehouseObject::MovableLeft | WarehouseObject::MovableRight) =
                ww.get(target_pos).unwrap()
            {
                target_pos = target_pos + instruction;
            }

            // println!("Next free space is {target_pos:?}, in front is {in_front:?}");

            // [][.
            if ww.get(target_pos).unwrap().is_none() {
                let x_min = (in_front + instruction).0.min(target_pos.0);
                let x_max = (in_front + instruction).0.max(target_pos.0);
                let mut even = true;
                for x in x_min..=x_max {
                    let intermediate: Point = (x, in_front.1).into();
                    if even {
                        // println!("Setting {intermediate:?} to [");
                        ww.set(intermediate, Some(WarehouseObject::MovableLeft));
                    } else {
                        // println!("Setting {intermediate:?} to ]");
                        ww.set(intermediate, Some(WarehouseObject::MovableRight));
                    }
                    even = !even;
                }
                robot_position = in_front;
                ww.set(robot_position, None);
            }
        }
    }
    ww
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
        let outputs = [2028usize];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part1(input).unwrap(), output);
        }
    }

    #[test]
    fn test_part2() {
        let inputs: Vec<Vec<String>> = [include_str!("../testcase_2.txt")]
            .iter()
            .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
            .collect();
        let outputs = [9021usize];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
    #[test]
    fn test_small_example() {
        let grid_input = vec![
            String::from("########"),
            String::from("#..O.O.#"),
            String::from("##@.O..#"),
            String::from("#...O..#"),
            String::from("#.#.O..#"),
            String::from("#...O..#"),
            String::from("#......#"),
            String::from("########"),
        ];
        let program_input = vec![String::from("<^^>>>vv<v>>v<<")];
        let (grid, r_pos) = parse_map(&grid_input);
        let program = parse_program(&program_input);
        assert!(r_pos.is_some());
        let result = simulate_warehouse(grid, program, r_pos.unwrap());
        let score = score_gps(result);
        assert_eq!(2028usize, score);
    }
    #[test]
    fn test_score_grid1() {
        let input = vec![
            String::from("#######"),
            String::from("#...O.."),
            String::from("#......"),
        ];
        let (grid, _) = parse_map(&input);
        let score = score_gps(grid);
        assert_eq!(104usize, score);
    }
    #[test]
    fn test_score_grid2() {
        let input = vec![
            String::from("########"),
            String::from("#....OO#"),
            String::from("##.....#"),
            String::from("#.....O#"),
            String::from("#.#O@..#"),
            String::from("#...O..#"),
            String::from("#...O..#"),
            String::from("########"),
        ];
        let (grid, _) = parse_map(&input);
        let score = score_gps(grid);
        assert_eq!(2028usize, score);
    }
    #[test]
    fn test_score_grid3() {
        let input = vec![
            String::from("##########"),
            String::from("#.O.O.OOO#"),
            String::from("#........#"),
            String::from("#OO......#"),
            String::from("#OO@.....#"),
            String::from("#O#.....O#"),
            String::from("#O.....OO#"),
            String::from("#O.....OO#"),
            String::from("#OO....OO#"),
            String::from("##########"),
        ];
        let (grid, _) = parse_map(&input);
        let score = score_gps(grid);
        assert_eq!(10092usize, score);
    }
}
