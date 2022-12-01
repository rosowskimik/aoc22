fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    let mut inner = Some(Vec::new());
    for line in input.lines() {
        if let Ok(n) = line.parse() {
            inner.get_or_insert_with(Vec::new).push(n);
        } else {
            result.push(inner.take().unwrap());
        }
    }

    result
}

fn part1(parsed: &[Vec<u32>]) -> u32 {
    parsed.iter().map(|inner| inner.iter().sum()).max().unwrap()
}

fn part2(parsed: &[Vec<u32>]) -> u32 {
    let mut best3 = [0u32; 3];

    parsed
        .iter()
        .map(|inner| inner.iter().sum())
        .for_each(|el| {
            if el >= best3[0] {
                best3[0] = el;
                best3.sort_unstable();
            }
        });

    best3.iter().sum()
}

fn main() {
    let parsed = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_input() -> Vec<Vec<u32>> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let input = parsed_input();
        let expected = 24000;

        let result = part1(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = parsed_input();
        let expected = 45000;

        let result = part2(&input);

        assert_eq!(expected, result);
    }
}
