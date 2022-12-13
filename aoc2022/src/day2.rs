use qu::ick_use::*;
use Res::*;
use RPS::*;

const INPUT: &str = include_str!("../input/2");

pub fn first() -> Result<u32> {
    let input = Round1::parse(INPUT)?;
    Ok(input
        .iter()
        .map(|round| round.opponent.result(round.us).score() + round.us.score())
        .sum())
}

pub fn second() -> Result<u32> {
    let input = Round2::parse(INPUT)?;
    Ok(input
        .iter()
        .map(|round| round.opponent.for_result(round.result).score() + round.result.score())
        .sum())
}

#[derive(Copy, Clone)]
struct Round1 {
    opponent: RPS,
    us: RPS,
}

impl Round1 {
    fn parse(input: &str) -> Result<Vec<Self>> {
        input
            .lines()
            .map(Round1::from_line)
            .collect::<Option<Vec<_>>>()
            .context("problem parsing input")
    }

    fn from_line(input: &str) -> Option<Self> {
        let input = input.trim();
        let mut iter = input.chars();
        let opponent = RPS::from_ch(iter.next()?)?;
        iter.next()?;
        let us = RPS::from_ch(iter.next()?)?;
        Some(Self { opponent, us })
    }
}

struct Round2 {
    opponent: RPS,
    result: Res,
}

impl Round2 {
    fn parse(input: &str) -> Result<Vec<Self>> {
        input
            .lines()
            .map(Round2::from_line)
            .collect::<Option<Vec<_>>>()
            .context("problem parsing input")
    }

    fn from_line(input: &str) -> Option<Self> {
        let input = input.trim();
        let mut iter = input.chars();
        let opponent = RPS::from_ch(iter.next()?)?;
        iter.next()?;
        let result = Res::from_char(iter.next()?)?;
        Some(Self { opponent, result })
    }
}

#[derive(Copy, Clone, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_ch(ch: char) -> Option<Self> {
        Some(match ch {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => return None,
        })
    }

    fn result(self, us: Self) -> Res {
        match (self, us) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            _ => Lose,
        }
    }

    /// If `self` is our opponent's choice, and we want the result `res`, what should we
    /// choose.
    fn for_result(self, res: Res) -> Self {
        match (self, res) {
            (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
            (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
            (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors,
        }
    }

    fn score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Copy, Clone)]
enum Res {
    Win,
    Lose,
    Draw,
}

impl Res {
    fn from_char(ch: char) -> Option<Self> {
        Some(match ch {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => return None,
        })
    }

    fn score(self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}
