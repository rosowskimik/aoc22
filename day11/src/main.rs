mod monkey;

use monkey::*;

use std::cmp::Reverse;

fn part1(mut m: Vec<Monkey>) -> usize {
    (0..20).for_each(|_| round(&mut m, 0));

    m.sort_unstable_by_key(|m| Reverse(m.inspected));

    m.into_iter().take(2).map(|m| m.inspected).product()
}

fn part2(mut m: Vec<Monkey>) -> usize {
    let divisor_product = m.iter().map(|m| m.divisor).product();

    (0..10000).for_each(|_| round(&mut m, divisor_product));

    m.sort_unstable_by_key(|m| Reverse(m.inspected));

    m.into_iter().take(2).map(|m| m.inspected).product()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Monkey> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let input = get_input();

        let expected = 10605;
        let result = part1(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();

        let expected = 2713310158;
        let result = part2(input);

        assert_eq!(expected, result);
    }
}
