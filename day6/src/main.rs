#![feature(array_windows)]

use std::collections::HashSet;

fn find_unique_len<const N: usize>(input: &str) -> usize {
    let mut hs: HashSet<u8> = HashSet::with_capacity(N);

    input
        .as_bytes()
        .array_windows::<N>()
        .map(|window| {
            hs.clear();
            hs.extend(window);
            hs.len()
        })
        .enumerate()
        .find_map(|(i, unique_count)| if unique_count == N { Some(i) } else { None })
        .unwrap()
        + N
}

fn part1(input: &str) -> usize {
    find_unique_len::<4>(input)
}

fn part2(input: &str) -> usize {
    find_unique_len::<14>(input)
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

        let expected = 7;
        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();

        let expected = 19;
        let result = part2(input);

        assert_eq!(expected, result);
    }
}
