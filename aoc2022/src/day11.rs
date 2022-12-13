use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, value},
    multi::many1,
    sequence::tuple,
    IResult,
};
use qu::ick_use::*;
use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("../input/11");

pub fn first() -> Result<u32> {
    let input = Input::parse(INPUT)?;
    for monkey in input {
        println!("{:#?}", monkey);
    }
    todo!()
}

pub fn second() -> Result<u32> {
    todo!()
}

#[derive(Debug)]
struct Input {
    starting_items: Vec<u32>,
    operation: Op,
    test_divisible_by: u32,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

impl Input {
    fn parse(mut i: &str) -> Result<Vec<Self>> {
        let mut input = vec![];
        while !i.is_empty() {
            let monkey;
            (i, monkey) =
                parse_monkey(i).map_err(|e| format_err!("couldn't parse monkey: {}", e))?;
            input.push(monkey);
        }
        Ok(input)
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Times(Operand),
    Plus(Operand),
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Value(u32),
    Old,
}

fn parse_monkey(i: &str) -> IResult<&str, Input> {
    let (i, _) = tuple((tag("Monkey "), digit1, tag(":\n  Starting items: ")))(i)?;
    let (i, starting_items) = many1(map_res(tuple((digit1, opt(tag(", ")))), |(num, _)| {
        <u32 as FromStr>::from_str(num)
    }))(i)?;
    let (i, _) = tag("\n  Operation: new = old ")(i)?;
    let (i, operation) = parse_op(i)?;
    let (i, _) = tag("\n  Test: divisible by ")(i)?;
    let (i, test_divisible_by) = map_res(digit1, FromStr::from_str)(i)?;
    let (i, _) = tag("\n    If true: throw to monkey ")(i)?;
    let (i, if_true_throw_to) = map_res(digit1, FromStr::from_str)(i)?;
    let (i, _) = tag("\n    If false: throw to monkey ")(i)?;
    let (i, if_false_throw_to) = map_res(digit1, FromStr::from_str)(i)?;
    let (i, _) = tag("\n")(i)?;
    let (i, _) = opt(tag("\n"))(i)?;
    Ok((
        i,
        Input {
            starting_items,
            operation,
            test_divisible_by,
            if_true_throw_to,
            if_false_throw_to,
        },
    ))
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    let (i, op) = alt((tag("*"), tag("+")))(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, operand) = alt((
        value(Operand::Old, tag("old")),
        map_res(digit1, |val| {
            let num = FromStr::from_str(val)?;
            Ok::<Operand, ParseIntError>(Operand::Value(num))
        }),
    ))(i)?;

    let op = match op {
        "*" => Op::Times(operand),
        "+" => Op::Plus(operand),
        _ => unreachable!(),
    };

    Ok((i, op))
}
