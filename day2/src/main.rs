#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
enum MatchResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl TryFrom<&str> for MatchResult {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn cmp_hands(self, rhs: Self) -> MatchResult {
        use Hand::{Paper, Rock, Scissors};

        match (self, rhs) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => MatchResult::Draw,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => MatchResult::Win,
            _ => MatchResult::Loss,
        }
    }

    fn to_get_result(self, match_result: MatchResult) -> Self {
        use Hand::{Paper, Rock, Scissors};
        use MatchResult::{Draw, Loss, Win};

        match (self, match_result) {
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Win) => Paper,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Win) => Scissors,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Win) => Rock,
        }
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .collect()
}

fn part1(parsed: &[(&str, &str)]) -> u16 {
    parsed
        .iter()
        .filter_map(|&(h1, h2)| Hand::try_from(h1).ok().zip(Hand::try_from(h2).ok()))
        .fold(0, |acc, (h1, h2)| acc + h2 as u16 + h2.cmp_hands(h1) as u16)
}

fn part2(parsed: &[(&str, &str)]) -> u16 {
    parsed
        .iter()
        .filter_map(|&(hand, result)| {
            Hand::try_from(hand)
                .ok()
                .zip(MatchResult::try_from(result).ok())
        })
        .fold(0, |acc, (hand, result)| {
            acc + hand.to_get_result(result) as u16 + result as u16
        })
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<(&'static str, &'static str)> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let input = get_input();
        let expected = 15;

        let result = part1(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();
        let expected = 12;

        let result = part2(&input);

        assert_eq!(expected, result);
    }
}
