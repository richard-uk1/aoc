use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt, recognize, value},
    sequence::tuple,
    IResult,
};
use qu::ick_use::*;
use std::fmt;

const INPUT: &str = include_str!("../input/10");

pub fn first() -> Result<i32> {
    let inst_iter = INPUT.lines().map(|line| Inst::parse(line).unwrap());
    let signal_strength = Cpu::sum_cycles_20_60_100_140_180_220(inst_iter);
    Ok(signal_strength)
}

pub fn second() -> Result<Screen> {
    let mut inst_iter = INPUT.lines().map(|line| Inst::parse(line).unwrap());
    let mut mach = Machine::new();
    for _ in 0..240 {
        mach.cycle(&mut inst_iter);
    }
    Ok(mach.screen)
}

struct Machine {
    screen: Screen,
    cpu: Cpu,
}

impl Machine {
    fn new() -> Self {
        Self {
            screen: Screen::new(),
            cpu: Cpu::new(),
        }
    }

    fn cycle(&mut self, inst: &mut impl Iterator<Item = Inst>) {
        let x = self.cpu.cycles % 40;
        let y = self.cpu.cycles / 40;
        if (self.cpu.regx - i32::try_from(x).unwrap()).abs() <= 1 {
            self.screen.set_pixel(x, y, true);
        }
        self.cpu.cycle(inst)
    }
}

pub struct Screen {
    // 40 x 6 = 240 bits = 30 bytes
    pixels: [u8; 30],
}

impl Screen {
    fn new() -> Self {
        Self { pixels: [0; 30] }
    }

    fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        let idx = y * 40 + x;
        let byte = idx / 8;
        let offset = idx % 8;

        let old = self.pixels[byte];

        if on {
            self.pixels[byte] = old | 1 << offset;
        } else {
            self.pixels[byte] = old & !(1 << offset);
        }
    }

    fn pixel(&self, x: usize, y: usize) -> bool {
        let idx = y * 40 + x;
        let byte_idx = idx / 8;
        let offset = idx % 8;
        let byte = self.pixels[byte_idx];
        ((byte >> offset) & 1) == 1
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                if self.pixel(x, y) {
                    write!(f, "#")
                } else {
                    write!(f, ".")
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Cpu {
    cycles: usize,
    regx: i32,
    /// If an addx instruction is on its second cycle, this will be `Some`.
    in_flight: Option<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycles: 0,
            regx: 1,
            in_flight: None,
        }
    }

    fn cycle(&mut self, inst_iter: &mut impl Iterator<Item = Inst>) {
        self.cycles += 1;
        if let Some(val) = self.in_flight.take() {
            self.regx += val;
            return;
        }
        match inst_iter.next().unwrap() {
            Inst::NoOp => (),
            Inst::AddX(val) => self.in_flight = Some(val),
        }
    }

    fn sum_cycles_20_60_100_140_180_220(mut inst: impl Iterator<Item = Inst>) -> i32 {
        let mut mach = Self::new();
        let mut acc = 0;
        for _ in 0..19 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 20;
        for _ in 0..40 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 60;
        for _ in 0..40 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 100;
        for _ in 0..40 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 140;
        for _ in 0..40 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 180;
        for _ in 0..40 {
            mach.cycle(&mut inst);
        }
        acc += mach.regx * 220;
        acc
    }
}

#[derive(Debug, Copy, Clone)]
enum Inst {
    NoOp,
    AddX(i32),
}

impl Inst {
    fn parse(i: &str) -> Result<Self> {
        let (_, inst) = line(i).map_err(|_| format_err!("failed to parse line"))?;
        Ok(inst)
    }
}

fn line(i: &str) -> IResult<&str, Inst> {
    all_consuming(alt((
        value(Inst::NoOp, tag("noop")),
        map(tuple((tag("addx "), number)), |(_, n)| Inst::AddX(n)),
    )))(i)
}

fn number(i: &str) -> IResult<&str, i32> {
    map_res(
        recognize(tuple((opt(tag("-")), digit1))),
        std::str::FromStr::from_str,
    )(i)
}

#[test]
fn test_with_example_input() {
    let acc = Cpu::sum_cycles_20_60_100_140_180_220(
        TEST_INPUT.lines().map(|line| Inst::parse(line).unwrap()),
    );
    assert_eq!(acc, 13140);
}

#[cfg(test)]
const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
