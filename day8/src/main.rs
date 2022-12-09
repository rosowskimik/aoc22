#![feature(iter_collect_into)]

use ndarray::{s, Array2};

fn parse_input(i: &str) -> Array2<u8> {
    let width = i.lines().next().unwrap().len();
    let height = i.lines().count();
    let mut it = i
        .lines()
        .flat_map(|line| line.chars().map(|c| c as u8 - b'0'));

    Array2::from_shape_simple_fn((width, height), || it.next().unwrap())
}

fn part1(i: &Array2<u8>) -> usize {
    let [w, h]: [_; 2] = i.shape().try_into().unwrap();
    let mut visible = Array2::from_shape_fn((w, h), |(x, y)| {
        x == 0 || y == 0 || x == w - 1 || y == h - 1
    });

    visible.indexed_iter_mut().for_each(|((x, y), val)| {
        *val = i.slice(s![..x, y]).iter().all(|&v| v < i[(x, y)])
            || i.slice(s![x + 1.., y]).iter().all(|&v| v < i[(x, y)])
            || i.slice(s![x, ..y]).iter().all(|&v| v < i[(x, y)])
            || i.slice(s![x, y + 1..]).iter().all(|&v| v < i[(x, y)])
    });

    visible.iter().filter(|&&v| v).count()
}

fn part2(i: &Array2<u8>) -> usize {
    let mut buf: Vec<u8> = Vec::new();

    i.indexed_iter()
        .map(|((x, y), &val)| {
            [s![..x, y], s![x + 1.., y], s![x, ..y], s![x, y + 1..]]
                .into_iter()
                .enumerate()
                .map(|(idx, s)| {
                    buf.clear();
                    i.slice(s).iter().collect_into(&mut buf);

                    if idx % 2 == 0 {
                        buf.reverse();
                    }

                    let mut counter = 0;
                    for &v in &buf {
                        counter += 1;
                        if v >= val {
                            break;
                        }
                    }
                    counter
                })
                .product()
        })
        // .inspect(|v| println!("{v}"))
        .max()
        .unwrap()
}

fn main() {
    let map = parse_input(include_str!("../input.txt"));

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Array2<u8> {
        parse_input(include_str!("../test.txt"))
    }

    #[test]
    fn p1() {
        let input = get_input();

        let expected = 21;
        let result = part1(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let input = get_input();

        let expected = 8;
        let result = part2(&input);

        assert_eq!(expected, result);
    }
}
