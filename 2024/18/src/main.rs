use anyhow::bail;
use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

use common::{
    grid::{
        coordinates::{Point, CARDINALS},
        Grid,
    },
    search::graph_search,
};

fn main() -> anyhow::Result<()> {
    let res = common::advent(part1, part2)?;
    println!("{res}");
    Ok(())
}

fn part1(input: Vec<String>) -> anyhow::Result<usize> {
    let block_list = input
        .iter()
        .map(|line| {
            Point::from(
                line.split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .next_tuple::<(i64, i64)>()
                    .unwrap(),
            )
        })
        .collect::<Vec<Point>>();
    let path_to_safety = path_through_grid(1024, 70, &block_list).unwrap();
    Ok(path_to_safety.len() - 1)
}

fn part2(input: Vec<String>) -> anyhow::Result<String> {
    let start = Instant::now();
    let block_list = input
        .iter()
        .map(|line| {
            Point::from(
                line.split(',')
                    .map(|n| n.parse::<i64>().unwrap())
                    .next_tuple::<(i64, i64)>()
                    .unwrap(),
            )
        })
        .collect::<Vec<Point>>();
    if let Ok(block) = blocking_block(70, &block_list) {
        let elapsed = start.elapsed();
        eprintln!("{}millis", elapsed.as_millis());
        Ok(block)
    } else {
        bail!("no block found");
    }
}

fn blocking_block(grid_size: usize, block_list: &[Point]) -> anyhow::Result<String> {
    let options = (0..block_list.len())
        .tuple_windows()
        .collect::<Vec<(usize, usize)>>();
    let straw = options.binary_search_by(|(a, b)| {
        match (
            path_through_grid(*a, grid_size, block_list),
            path_through_grid(*b, grid_size, block_list),
        ) {
            (Some(_), Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Equal,
            (None, None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => unreachable!(),
        }
    });

    if let Ok(position) = straw {
        let last_block = block_list[position];
        Ok(format!("{},{}", last_block.0, last_block.1))
    } else {
        bail!("Couldn't find the block")
    }
}

fn path_through_grid(
    blocks_to_drop: usize,
    grid_size: usize,
    block_list: &[Point],
) -> Option<Vec<Point>> {
    let mut memory_space: Grid<Option<()>> = Grid::from_parts(
        vec![None; (grid_size + 1) * (grid_size + 1)],
        grid_size + 1,
        grid_size + 1,
    );
    for point in &block_list[0..blocks_to_drop] {
        memory_space.set(*point, Some(()));
    }

    let shortest_path = graph_search(
        Point(0, 0),
        |&point| point == Point(grid_size as i64, grid_size as i64),
        |&point| {
            let mut next = Vec::new();
            for &cardinal in CARDINALS {
                let potential = point + cardinal;
                if let Some(in_space) = memory_space.get(potential) {
                    if in_space.is_none() {
                        next.push((potential, 1));
                    }
                }
            }
            next
        },
        &mut VecDeque::new(),
    );
    shortest_path.map(|some_path| some_path.path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let block_list = &[
            Point(5i64, 4i64),
            Point(4i64, 2i64),
            Point(4i64, 5i64),
            Point(3i64, 0i64),
            Point(2i64, 1i64),
            Point(6i64, 3i64),
            Point(2i64, 4i64),
            Point(1i64, 5i64),
            Point(0i64, 6i64),
            Point(3i64, 3i64),
            Point(2i64, 6i64),
            Point(5i64, 1i64),
            Point(1i64, 2i64),
            Point(5i64, 5i64),
            Point(2i64, 5i64),
            Point(6i64, 5i64),
            Point(1i64, 4i64),
            Point(0i64, 4i64),
            Point(6i64, 4i64),
            Point(1i64, 1i64),
            Point(6i64, 1i64),
            Point(1i64, 0i64),
            Point(0i64, 5i64),
            Point(1i64, 6i64),
            Point(2i64, 0i64),
        ];
        let path_to_safety = path_through_grid(12, 6, block_list).unwrap();
        assert_eq!(path_to_safety.len() - 1, 22);
    }

    #[test]
    fn test_part2() {
        let block_list = &[
            Point(5i64, 4i64),
            Point(4i64, 2i64),
            Point(4i64, 5i64),
            Point(3i64, 0i64),
            Point(2i64, 1i64),
            Point(6i64, 3i64),
            Point(2i64, 4i64),
            Point(1i64, 5i64),
            Point(0i64, 6i64),
            Point(3i64, 3i64),
            Point(2i64, 6i64),
            Point(5i64, 1i64),
            Point(1i64, 2i64),
            Point(5i64, 5i64),
            Point(2i64, 5i64),
            Point(6i64, 5i64),
            Point(1i64, 4i64),
            Point(0i64, 4i64),
            Point(6i64, 4i64),
            Point(1i64, 1i64),
            Point(6i64, 1i64),
            Point(1i64, 0i64),
            Point(0i64, 5i64),
            Point(1i64, 6i64),
            Point(2i64, 0i64),
        ];
        let block = blocking_block(6, block_list).unwrap();
        assert_eq!(block, "6,1");
    }
}
