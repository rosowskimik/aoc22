use std::{
    collections::VecDeque,
    fmt::{self, Write},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::{all_consuming, map, value},
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn parse(s: &str) -> IResult<&str, Self> {
        let addx = map(preceded(tag("addx "), complete::i32), Self::Addx);
        let noop = value(Self::Noop, tag("noop"));
        alt((addx, noop))(s)
    }

    fn cycles(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    instructions: VecDeque<Instruction>,
    current: Option<(Instruction, usize)>,
    cycle: usize,
    x: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            instructions: VecDeque::new(),
            current: None,
            cycle: 1,
            x: 1,
        }
    }

    fn reset(&mut self) {
        self.instructions.clear();
        self.current = None;
        self.cycle = 1;
        self.x = 1;
    }

    fn load_program(&mut self, program: &[Instruction]) {
        if program.is_empty() {
            return;
        }
        self.current = Some((program[0], program[0].cycles()));
        self.instructions.extend(&program[1..]);
    }

    fn finished(&self) -> bool {
        self.current.is_none()
    }

    fn run_cycle(&mut self) {
        if self.finished() {
            return;
        }
        self.cycle += 1;

        let (i, c) = self.current.as_mut().unwrap();
        if *c > 1 {
            *c -= 1;
            return;
        }

        match *i {
            Instruction::Addx(v) => {
                self.x += v;
            }
            Instruction::Noop => {}
        }

        self.current = self.instructions.pop_front().map(|i| (i, i.cycles()));
    }
}

#[derive(Debug)]
struct Screen([[char; 40]; 6]);

impl Screen {
    fn new() -> Self {
        Self([['.'; 40]; 6])
    }

    fn set(&mut self, i: usize) {
        let (d, r) = (i / 40, i % 40);
        self.0[d][r] = '#';
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().enumerate().try_for_each(|(i, row)| {
            row.iter().try_for_each(|&c| f.write_char(c))?;
            if i < 5 {
                f.write_char('\n')?;
            }
            Ok(())
        })
    }
}

fn parse_input(i: &str) -> Vec<Instruction> {
    i.lines()
        .map(|l| all_consuming(Instruction::parse)(l).unwrap().1)
        .collect()
}

fn part1(cpu: &mut Cpu, instr: &[Instruction]) -> i32 {
    cpu.load_program(instr);

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|target| {
            while cpu.cycle != target as usize {
                cpu.run_cycle();
            }
            target * cpu.x
        })
        .sum()
}

fn part2(cpu: &mut Cpu, instr: &[Instruction]) -> String {
    cpu.reset();
    cpu.load_program(instr);

    let mut screen = Screen::new();

    let mut screen_offset = cpu.cycle - 1;
    screen.set(screen_offset);

    while !cpu.finished() {
        if cpu.x.abs_diff((screen_offset % 40) as i32) <= 1 {
            screen.set(screen_offset);
        }

        cpu.run_cycle();
        screen_offset += 1;
    }
    screen.to_string()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    let mut cpu = Cpu::new();

    println!("Part 1: {}", part1(&mut cpu, &input));
    println!("Part 2:\n{}", part2(&mut cpu, &input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> (Cpu, Vec<Instruction>) {
        (Cpu::new(), parse_input(include_str!("../test.txt")))
    }

    #[test]
    fn p1() {
        let (mut c, i) = get_input();

        let expected = 13140;
        let result = part1(&mut c, &i);

        assert_eq!(expected, result);
    }

    #[test]
    fn p2() {
        let (mut c, i) = get_input();

        let expected = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
            .to_string();
        let result = part2(&mut c, &i);

        assert_eq!(expected, result);
    }
}
