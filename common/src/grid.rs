use std::collections::HashMap;

use coordinates::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    storage: Vec<T>,
}

impl Grid<char> {
    pub fn from_input(input: Vec<String>, locate: &[char]) -> (Self, HashMap<char, Vec<Point>>) {
        let mut storage = Vec::new();
        let mut height = 0;
        let mut location_list: HashMap<char, Vec<Point>> = HashMap::new();
        for row in input {
            for (x, c) in row.chars().enumerate() {
                storage.push(c);
                if locate.contains(&c) {
                    location_list
                        .entry(c)
                        .or_default()
                        .push(Point::from((x, height)));
                }
            }
            height += 1;
        }
        let width = storage.len() / height;
        eprintln!("Initialized grid with width: {width}, height: {height}");
        (
            Self {
                width,
                height,
                storage,
            },
            location_list,
        )
    }
}

impl<T> Grid<T> {
    pub fn from_iterators<I, II>(input: I) -> Self
    where
        I: Iterator<Item = II>,
        II: Iterator<Item = T>,
    {
        let mut storage = Vec::new();
        let mut height = 0;
        for row in input {
            height += 1;
            storage.extend(row);
        }
        let width = storage.len() / height;
        eprintln!("Initialized grid with width: {width}, height: {height}");
        Self {
            width,
            height,
            storage,
        }
    }

    pub fn from_parts(storage: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            storage,
        }
    }

    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let height = rows.len();
        let width = rows.first().map(|row| row.len()).unwrap_or_default();
        let storage = rows.into_iter().flatten().collect();
        Self {
            width,
            height,
            storage,
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        coordinates::griderator(0..self.width as i64, 0..self.height as i64)
    }

    pub fn get<IP>(&self, point: IP) -> Option<&T>
    where
        IP: Into<Point>,
    {
        let Point(x, y) = point.into();
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        self.storage.get((x as usize) + (self.width * (y as usize)))
    }

    pub fn set<IP>(&mut self, point: IP, val: T) -> Option<&T>
    where
        IP: Into<Point>,
    {
        let Point(x, y) = point.into();
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return None;
        }
        let index = (x as usize) + (self.width * (y as usize));
        self.storage[index] = val;
        self.storage.get(index)
    }
}

pub mod coordinates {
    use std::ops::{Add, Mul};
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Point(pub i64, pub i64);

    impl Point {
        pub fn dist(&self, rhs: Self) -> usize {
            (self.0.abs_diff(rhs.0) + self.1.abs_diff(rhs.1)) as usize
        }
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, rhs: Self) -> Self::Output {
            Point(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Mul<i64> for Point {
        type Output = Point;
        fn mul(self, rhs: i64) -> Self::Output {
            Point(self.0 * rhs, self.1 * rhs)
        }
    }

    impl From<(i64, i64)> for Point {
        fn from(value: (i64, i64)) -> Self {
            Self(value.0, value.1)
        }
    }

    impl From<(i32, i32)> for Point {
        fn from(value: (i32, i32)) -> Self {
            Self(value.0 as i64, value.1 as i64)
        }
    }

    impl From<(usize, usize)> for Point {
        fn from(value: (usize, usize)) -> Self {
            Self(value.0 as i64, value.1 as i64)
        }
    }

    pub const NORTH: Point = Point(0, -1);
    pub const NORTH_WEST: Point = Point(-1, -1);
    pub const WEST: Point = Point(-1, 0);
    pub const SOUTH_WEST: Point = Point(-1, 1);
    pub const SOUTH: Point = Point(0, 1);
    pub const SOUTH_EAST: Point = Point(1, 1);
    pub const EAST: Point = Point(1, 0);
    pub const NORTH_EAST: Point = Point(1, -1);

    pub const UNIT: &[Point] = &[
        NORTH, NORTH_WEST, WEST, SOUTH_WEST, SOUTH, SOUTH_EAST, EAST, NORTH_EAST,
    ];
    pub const CARDINALS: &[Point] = &[NORTH, WEST, SOUTH, EAST];

    pub fn griderator<I>(range_x: I, range_y: I) -> impl Iterator<Item = Point>
    where
        I: Iterator<Item = i64> + Clone,
    {
        range_y.flat_map(move |y| range_x.clone().map(move |x| Point(x, y)))
    }
}
