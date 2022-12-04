#![feature(result_flattening)]
#![feature(type_alias_impl_trait)]

use anyhow::{anyhow, Error};

use std::str::FromStr;

#[derive(Debug, Clone)]
struct Assignment {
    start: u8,
    end: u8,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl FromStr for Assignment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once('-')
            .ok_or(anyhow!("input does not contain '-'"))?;

        Ok(Self {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .filter_map(|line| line.split_once(','))
        .filter_map(|(a1, a2)| {
            Assignment::from_str(a1)
                .and_then(|v1| Assignment::from_str(a2).map(|v2| (v1, v2)))
                .ok()
        })
        .collect()
}

fn part1(input: &[(Assignment, Assignment)]) -> usize {
    input
        .iter()
        .filter(|(a1, a2)| a1.contains(a2) || a2.contains(a1))
        .count()
}

fn part2(input: &[(Assignment, Assignment)]) -> usize {
    input.iter().filter(|(a1, a2)| a1.overlaps(a2)).count()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<(Assignment, Assignment)> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let input = get_input();

        let expected = 2;
        let result = part1(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();

        let expected = 4;
        let result = part2(&input);

        assert_eq!(expected, result);
    }
}
