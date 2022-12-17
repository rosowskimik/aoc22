use std::{collections::VecDeque, mem};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0, multispace1, one_of, space1},
    combinator::map,
    combinator::{all_consuming, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    AsChar, Finish, IResult, InputTakeAtPosition, Parser,
};

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Old,
    Const(u64),
}

impl Operand {
    pub fn value(&self, old: u64) -> u64 {
        match *self {
            Self::Const(v) => v,
            Self::Old => old,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl Operation {
    pub fn eval(&self, old: u64) -> u64 {
        match *self {
            Self::Add(x, y) => x.value(old) + y.value(old),
            Self::Mul(x, y) => x.value(old) * y.value(old),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub target_if_true: u64,
    pub target_if_false: u64,
    pub inspected: usize,
}

pub fn round(monkeys: &mut [Monkey], divisor_product: u64) {
    let mut buf = Monkey {
        items: VecDeque::new(),
        operation: Operation::Add(Operand::Old, Operand::Old),
        divisor: 0,
        target_if_true: 0,
        target_if_false: 0,
        inspected: 0,
    };

    (0..monkeys.len()).for_each(|curr| {
        mem::swap(&mut buf, &mut monkeys[curr]);
        let Monkey {
            items,
            operation,
            divisor,
            target_if_true,
            target_if_false,
            inspected,
        } = &mut buf;

        while let Some(item) = items.pop_front() {
            *inspected += 1;

            let mut item = operation.eval(if divisor_product != 0 {
                item % divisor_product
            } else {
                item
            });
            if divisor_product == 0 {
                item /= 3;
            }

            let target = if item % *divisor == 0 {
                *target_if_true
            } else {
                *target_if_false
            };

            monkeys[target as usize].items.push_back(item);
        }
        mem::swap(&mut buf, &mut monkeys[curr]);
    });
}

fn indented<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Parser<I, O, E>,
{
    preceded(space1, parser)
}

fn parse_operand(i: &str) -> IResult<&str, Operand> {
    alt((
        value(Operand::Old, tag("old")),
        map(complete::u64, Operand::Const),
    ))(i)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (x, op, y)) = tuple((
        parse_operand,
        delimited(space1, one_of("+*"), space1),
        parse_operand,
    ))(i)?;

    let op = match op {
        '+' => Operation::Add(x, y),
        '*' => Operation::Mul(x, y),
        _ => unreachable!(),
    };

    Ok((i, op))
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), complete::u64, tag(":\n")))(i)?;
    let (i, items) = delimited(
        indented(tag("Starting items: ")),
        map(separated_list1(tag(", "), complete::u64), VecDeque::from),
        tag("\n"),
    )(i)?;
    let (i, operation) = delimited(
        indented(tag("Operation: new = ")),
        parse_operation,
        tag("\n"),
    )(i)?;
    let (i, divisor) = delimited(
        indented(tag("Test: divisible by ")),
        complete::u64,
        tag("\n"),
    )(i)?;
    let (i, target_if_true) = delimited(
        indented(tag("If true: throw to monkey ")),
        complete::u64,
        tag("\n"),
    )(i)?;
    let (i, target_if_false) =
        preceded(indented(tag("If false: throw to monkey ")), complete::u64)(i)?;

    Ok((
        i,
        Monkey {
            items,
            operation,
            divisor,
            target_if_true,
            target_if_false,
            inspected: 0,
        },
    ))
}

pub fn parse_input(i: &str) -> Vec<Monkey> {
    all_consuming(terminated(
        separated_list1(multispace1, parse_monkey),
        multispace0,
    ))(i)
    .finish()
    .unwrap()
    .1
}
