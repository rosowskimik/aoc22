use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self(x, y)
    }
}

impl From<Point> for (i32, i32) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    count: usize,
}

#[derive(Debug, Clone)]
struct Line<const N: usize> {
    body: [Point; N],
}

impl<const N: usize> Line<N> {
    fn new() -> Self {
        Self {
            body: [Point::default(); N],
        }
    }

    fn move_line(&mut self, d: Direction) {
        let op = match d {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };

        self.body[0] += op.into();

        (0..N).tuple_windows().for_each(|(f, s)| {
            let change = match (self.body[f] - self.body[s]).into() {
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                (-2, 1) | (-1, 2) | (-2, 2) => (-1, 1),
                (2, 1) | (1, 2) | (2, 2) => (1, 1),
                (2, -1) | (1, -2) | (2, -2) => (1, -1),
                (-2, -1) | (-1, -2) | (-2, -2) => (-1, -1),
                _ => (0, 0),
            }
            .into();
            self.body[s] += change;
        });
    }
}

fn parse_input(i: &str) -> Vec<Move> {
    i.lines()
        .map(|line| {
            let direction = match line.as_bytes()[0] {
                b'U' => Direction::Up,
                b'D' => Direction::Down,
                b'R' => Direction::Right,
                b'L' => Direction::Left,
                _ => unreachable!(),
            };

            let count = line[2..].parse().unwrap();

            Move { direction, count }
        })
        .collect()
}

fn part1(i: &[Move]) -> usize {
    let mut hs = HashSet::new();
    let mut line = Line::<2>::new();

    i.iter().for_each(|m| {
        (0..m.count).for_each(|_| {
            line.move_line(m.direction);
            hs.insert(line.body[1]);
        });
    });

    hs.len()
}

fn part2(i: &[Move]) -> usize {
    let mut hs = HashSet::new();
    let mut line = Line::<10>::new();

    i.iter().for_each(|m| {
        (0..m.count).for_each(|_| {
            line.move_line(m.direction);
            hs.insert(line.body[9]);
        });
    });

    hs.len()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let i = parse_input(include_str!("../test.txt"));

        let expected = 13;
        let result = part1(&i);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let i = parse_input(include_str!("../test2.txt"));

        let expected = 36;
        let result = part2(&i);

        assert_eq!(expected, result);
    }
}
