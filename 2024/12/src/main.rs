use std::{collections::HashSet, time::Instant};

use common::grid::{
    coordinates::{Point, EAST, NORTH, SOUTH, WEST},
    Grid,
};
fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let result = common::advent(part1, part2)?;
    let complete = start.elapsed();
    println!("got result {result} in {}", complete.as_millis());
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions = Vec::new();
    let grid = Grid::from_iterators(input.iter().map(|line| line.chars()));
    for point in grid.points() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        let garden_type = *grid.get(point).unwrap();
        let mut region = vec![point];
        let neighbours = [NORTH, EAST, WEST, SOUTH];
        let mut next_to_visit = Vec::from_iter(neighbours.iter().map(|n| *n + point));
        while let Some(location) = next_to_visit.pop() {
            if visited.contains(&location) {
                continue;
            }
            if let Some(plant) = grid.get(location) {
                if *plant == garden_type {
                    visited.insert(location);
                    region.push(location);
                    next_to_visit.extend(neighbours.iter().map(|n| *n + location));
                }
            }
        }
        regions.push(region);
    }

    let mut total = 0;
    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        for location in &region {
            for neighbour in [NORTH, EAST, WEST, SOUTH] {
                if !region.contains(&(neighbour + *location)) {
                    perimeter += 1;
                }
            }
        }
        total += perimeter * area;
    }
    Ok(total)
}

fn part2(input: Vec<String>) -> anyhow::Result<usize> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions = Vec::new();
    let grid = Grid::from_iterators(input.iter().map(|line| line.chars()));
    for point in grid.points() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        let garden_type = *grid.get(point).unwrap();
        let mut region = vec![point];
        let neighbours = [NORTH, EAST, WEST, SOUTH];
        let mut next_to_visit = Vec::from_iter(neighbours.iter().map(|n| *n + point));
        while let Some(location) = next_to_visit.pop() {
            if visited.contains(&location) {
                continue;
            }
            if let Some(plant) = grid.get(location) {
                if *plant == garden_type {
                    visited.insert(location);
                    region.push(location);
                    next_to_visit.extend(neighbours.iter().map(|n| *n + location));
                }
            }
        }
        regions.push(region);
    }

    let mut total = 0;
    for region in regions {
        let area = region.len();
        let garden_type = grid.get(region[0]).unwrap();

        let mut min_x = grid.width as i64;
        let mut min_y = grid.height as i64;
        let mut max_x = 0;
        let mut max_y = 0;
        for point in &region {
            min_x = min_x.min(point.0);
            min_y = min_y.min(point.1);
            max_x = max_x.max(point.0);
            max_y = max_y.max(point.1);
        }

        let mut edges = 0;
        // horizontally travelling edges
        for y in min_y..=max_y {
            let mut on_edge = false;
            let mut on_edge_bottom = false;
            for x in min_x..=max_x {
                let point: Point = (x, y).into();
                if region.contains(&point) && !region.contains(&(point + NORTH))
                    || !region.contains(&point) && region.contains(&(point + NORTH))
                {
                    if !on_edge {
                        // if *garden_type == 'A' {
                        //     println!("Found horizontal edge starting at {point:?}");
                        // }
                        edges += 1;
                    } else if region.contains(&point) && !region.contains(&(point + WEST))
                        || !region.contains(&point) && region.contains(&(point + WEST))
                    {
                        edges += 1;
                    }
                    on_edge = true;
                } else {
                    on_edge = false;
                }

                if y == max_y && region.contains(&point) && !region.contains(&(point + SOUTH)) {
                    // if *garden_type == 'A' {
                    //     println!("Found horizontal bottom edge starting at {point:?}");
                    // }
                    if !on_edge_bottom {
                        edges += 1;
                    }
                    on_edge_bottom = true;
                } else {
                    on_edge_bottom = false;
                }
            }
        }
        // vertically travelling edges
        for x in min_x..=max_x {
            let mut on_edge = false;
            let mut on_edge_east = false;
            for y in min_y..=max_y {
                let point: Point = (x, y).into();
                if region.contains(&point) && !region.contains(&(point + WEST))
                    || !region.contains(&point) && region.contains(&(point + WEST))
                {
                    if !on_edge {
                        // if *garden_type == 'A' {
                        //     println!("Found vertical edge starting at {point:?}");
                        // }
                        edges += 1;
                    } else if region.contains(&point) && !region.contains(&(point + NORTH))
                        || !region.contains(&point) && region.contains(&(point + NORTH))
                    {
                        edges += 1;
                    }
                    on_edge = true;
                } else {
                    on_edge = false;
                }

                if x == max_x && region.contains(&point) && !region.contains(&(point + EAST)) {
                    if !on_edge_east {
                        // if *garden_type == 'A' {
                        //     println!("Found vertical eastern edge starting at {point:?}");
                        // }
                        edges += 1;
                    }
                    on_edge_east = true;
                } else {
                    on_edge_east = false;
                }
            }
        }
        // println!(
        //     "Found {edges} edges on an {area} sized block of {garden_type} starting at {:?}",
        //     region[0]
        // );
        total += edges * area;
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
        let outputs = [1930];
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
            include_str!("../testcase_3.txt"),
            include_str!("../testcase_4.txt"),
            include_str!("../testcase_5.txt"),
        ]
        .iter()
        .map(|input| input.lines().map(String::from).collect::<Vec<String>>())
        .collect();
        let outputs = [1206, 80, 236, 368, 436];
        assert_eq!(inputs.len(), outputs.len());
        for (input, &output) in inputs.into_iter().zip(outputs.iter()) {
            assert_eq!(part2(input).unwrap(), output);
        }
    }
}
