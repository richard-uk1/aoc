use nom::{bytes::complete::tag, character::complete::digit1, Finish, IResult};
use qu::ick_use::*;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/4");

pub fn first() -> Result<u32> {
    let input = Input::parse(INPUT)?;
    let count_contains = input.iter().filter(|i| i.fully_contains()).count();
    Ok(count_contains.try_into().unwrap())
}

pub fn second() -> Result<u32> {
    let input = Input::parse(INPUT)?;
    let count_overlap = input.iter().filter(|i| i.ranges_overlap()).count();
    Ok(count_overlap.try_into().unwrap())
}

struct Input {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

impl Input {
    fn parse(input: &str) -> Result<Vec<Input>> {
        input
            .lines()
            .map(Input::parse_row)
            .collect::<Result<Vec<_>>>()
    }

    fn parse_row(input: &str) -> Result<Input> {
        let (_, input) = row(input).finish().unwrap();
        Ok(input)
    }

    /// One of the ranges fully contains the other
    fn fully_contains(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    fn ranges_overlap(&self) -> bool {
        self.first.end() >= self.second.start() && self.second.end() >= self.first.start()
    }
}

// parse

fn row(i: &str) -> IResult<&str, Input> {
    let (i, first) = range(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, second) = range(i)?;
    Ok((i, Input { first, second }))
}

fn range(i: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (i, start) = number(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, end) = number(i)?;
    Ok((i, RangeInclusive::new(start, end)))
}

fn number(i: &str) -> IResult<&str, u32> {
    let (i, num) = digit1(i)?;
    let num = num.parse::<u32>().unwrap();
    Ok((i, num))
}
