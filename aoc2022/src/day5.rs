use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res, opt, value},
    IResult,
};
use qu::ick_use::*;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/5");

pub fn first() -> Result<String> {
    let Input {
        mut start,
        instructions,
    } = Input::parse(INPUT)?;
    for inst in instructions {
        start.apply_instruction(inst);
    }
    Ok(start.top_crates())
}

pub fn second() -> Result<String> {
    let Input {
        mut start,
        instructions,
    } = Input::parse(INPUT)?;
    for inst in instructions {
        start.apply_instruction_9001(inst);
    }
    Ok(start.top_crates())
}

#[derive(Debug)]
struct Input {
    start: CratePositions,
    instructions: Vec<Instruction>,
}

impl Input {
    fn parse(i: &str) -> Result<Self> {
        let mut lines = i.lines().peekable();
        let mut start = CratePositions::new();
        while !lines.peek().unwrap().starts_with(" 1") {
            let line = lines.next().unwrap();
            start.parse_line(line)?;
        }
        // skip numbers line and blank line after it
        lines.next();
        lines.next();
        // parse instructions
        let mut instructions = vec![];
        for line in lines {
            let (_, instruction) = Instruction::parse(line)
                .map_err(|_| format_err!("couldn't parse instruction line"))?;
            instructions.push(instruction);
        }
        // Make it so the first popped el of the vecs is the top of the column
        for column in &mut start.columns {
            column.reverse();
        }
        Ok(Self {
            start,
            instructions,
        })
    }
}

#[derive(Debug)]
struct CratePositions {
    columns: Vec<Vec<Crate>>,
}

impl CratePositions {
    fn new() -> Self {
        Self { columns: vec![] }
    }

    /// Ensure we have at least `len` columns.
    fn ensure_len(&mut self, len: usize) {
        if self.columns.len() < len {
            self.columns.resize(len, vec![]);
        }
    }

    fn parse_line(&mut self, mut i: &str) -> Result {
        let mut krate;
        let mut column = 0;
        while !i.is_empty() {
            (i, krate) = Crate::parse(i).map_err(|_| format_err!("couldn't parse [<char>]"))?;
            self.ensure_len(column + 1);
            if let Some(krate) = krate {
                self.columns[column].push(krate);
            }
            column += 1;
        }
        Ok(())
    }

    fn apply_instruction(&mut self, inst: Instruction) {
        let from = inst.from();
        let to = inst.to();
        for _ in 0..inst.count {
            let krate = self.columns[from].pop().unwrap();
            self.columns[to].push(krate);
        }
    }

    fn apply_instruction_9001(&mut self, inst: Instruction) {
        let from = inst.from();
        let to = inst.to();
        assert!(from != to);
        let count = inst.count;

        let from_len = self.columns[from].len();
        for i in from_len - count..from_len {
            let val = self.columns[from][i];
            self.columns[to].push(val);
        }
        self.columns[from].truncate(from_len - count);
    }

    fn top_crates(&self) -> String {
        let mut out = String::new();
        for column in &self.columns {
            out.push(column[column.len() - 1].0);
        }
        out
    }
}

#[derive(Copy, Clone, Debug)]
struct Crate(char);

impl Crate {
    fn parse(i: &str) -> IResult<&str, Option<Self>> {
        alt((
            map(Self::parse_crate, |v| Some(v)),
            value(None, Self::parse_space),
        ))(i)
    }
    fn parse_crate(i: &str) -> IResult<&str, Self> {
        let (i, _) = tag("[")(i)?;
        let (i, ch) = anychar(i)?;
        let (i, _) = tag("]")(i)?;
        let (i, _) = opt(tag(" "))(i)?;
        Ok((i, Crate(ch)))
    }

    fn parse_space(i: &str) -> IResult<&str, ()> {
        let (i, _) = tag("   ")(i)?;
        let (i, _) = opt(tag(" "))(i)?;
        Ok((i, ()))
    }
}

/// Remember that we store columns 0-indexed, but they are recorded here as 1-indexed.
#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Instruction> {
        let i = i.trim();
        let (i, _) = tag("move ")(i)?;
        let (i, count): (&str, usize) = map_res(digit1, FromStr::from_str)(i)?;
        let (i, _) = tag(" from ")(i)?;
        let (i, from): (&str, usize) = map_res(digit1, FromStr::from_str)(i)?;
        let (i, _) = tag(" to ")(i)?;
        let (i, to): (&str, usize) = map_res(digit1, FromStr::from_str)(i)?;
        Ok((i, Instruction { count, from, to }))
    }

    /// Get 0-indexed `from` value.
    fn from(&self) -> usize {
        self.from - 1
    }

    /// Get 0-indexed `to` value.
    fn to(&self) -> usize {
        self.to - 1
    }
}
