#![feature(never_type)]

use std::{mem, str::FromStr};

#[derive(Debug, Clone)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn move_crates_by_one(&mut self, m: &CraneMove) {
        for _ in 0..m.count {
            let c = self.stacks[m.from - 1].pop().unwrap();
            self.stacks[m.to - 1].push(c);
        }
    }

    fn move_crates(&mut self, m: &CraneMove) {
        let mut from = mem::take(&mut self.stacks[m.from - 1]);

        self.stacks[m.to - 1].extend(from.drain((from.len() - m.count)..));
        self.stacks[m.from - 1] = from;
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last())
            .collect::<Option<String>>()
            .unwrap_or_default()
    }
}

impl FromStr for Crates {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().rev();

        let mut stacks = vec![Vec::new(); it.next().unwrap().len() / 4 + 1];

        it.for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphabetic())
                .for_each(|(i, c)| stacks[i].push(c));
        });

        Ok(Self { stacks })
    }
}

#[derive(Debug)]
struct CraneMove {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for CraneMove {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|n| n.parse().unwrap());

        Ok(Self {
            count: it.next().unwrap(),
            from: it.next().unwrap(),
            to: it.next().unwrap(),
        })
    }
}

fn parse_input(input: &str) -> (Crates, Vec<CraneMove>) {
    let (fst, snd) = input.split_at(
        input
            .find("\n\n")
            .or_else(|| input.find("\r\n\r\n"))
            .unwrap(),
    );

    (
        Crates::from_str(fst).unwrap(),
        snd.trim_start()
            .lines()
            .map(CraneMove::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
    )
}

fn part1(mut crates: Crates, moves: &[CraneMove]) -> String {
    for m in moves {
        crates.move_crates_by_one(m);
    }

    crates.top_crates()
}

fn part2(mut crates: Crates, moves: &[CraneMove]) -> String {
    for m in moves {
        crates.move_crates(m);
    }

    crates.top_crates()
}

fn main() {
    let (crates, moves) = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(crates.clone(), &moves));
    println!("Part 1: {}", part2(crates, &moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Crates, Vec<CraneMove>) {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let (crates, moves) = get_input();

        let expected = "CMZ";
        let result = part1(crates, &moves);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let (crates, moves) = get_input();

        let expected = "MCD";
        let result = part2(crates, &moves);

        assert_eq!(expected, result);
    }
}
