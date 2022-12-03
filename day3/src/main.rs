#![feature(iter_array_chunks)]

use std::collections::HashSet;

// <a, z> -> <1, 26>
// <A, Z> -> <27, 52>
fn char_priority(c: char) -> u16 {
    match c {
        // 'a' as u8 = 97
        'a'..='z' => c as u8 - 96,
        // 'A' as u8 == 65
        'A'..='Z' => c as u8 - 38,
        _ => unreachable!(),
    }
    .into()
}

fn part1(input: &str) -> u16 {
    let mut hs = HashSet::new();

    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .filter_map(|(c1, c2)| {
            hs.clear();
            hs.extend(c1.chars());

            c2.chars().find(|c| hs.contains(c))
        })
        .map(char_priority)
        .sum()
}

fn part2(input: &str) -> u16 {
    let mut h1 = HashSet::new();
    let mut h2 = h1.clone();
    let mut h3 = h1.clone();

    input
        .lines()
        .array_chunks::<3>()
        .filter_map(|group| {
            h1.clear();
            h2.clear();
            h3.clear();

            h1.extend(group[0].chars());
            h2.extend(group[1].chars());
            h3.extend(h1.intersection(&h2));

            group[2].chars().find(|c| h3.contains(c))
        })
        .map(char_priority)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        include_str!("../test.txt")
    }

    #[test]
    fn p1() {
        let input = get_input();
        let expected = 157;

        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();
        let expected = 70;

        let result = part2(input);

        assert_eq!(expected, result);
    }
}
