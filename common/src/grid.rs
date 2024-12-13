use coordinates::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    storage: Vec<T>,
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
}

pub mod coordinates {
    use std::ops::{Add, Mul};
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Point(pub i64, pub i64);

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

    impl From<(usize, usize)> for Point {
        fn from(value: (usize, usize)) -> Self {
            Self(value.0 as i64, value.1 as i64)
        }
    }

    pub const NORTH: Point = Point(0, -1);
    pub const NORTH_WEST: Point = Point(1, -1);
    pub const WEST: Point = Point(1, 0);
    pub const SOUTH_WEST: Point = Point(1, 1);
    pub const SOUTH: Point = Point(0, 1);
    pub const SOUTH_EAST: Point = Point(-1, 1);
    pub const EAST: Point = Point(-1, 0);
    pub const NORTH_EAST: Point = Point(-1, -1);

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
