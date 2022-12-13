use qu::ick_use::*;

const INPUT: &str = include_str!("../input/3");

pub fn first() -> Result<u32> {
    let input = Input::parse(INPUT)?;
    Ok(input
        .iter()
        .map(|input| input.find_match_1().unwrap().score())
        .sum::<u32>())
}

pub fn second() -> Result<u32> {
    let input = Input::parse(INPUT)?;
    Ok(input
        .chunks(3)
        .map(|chunk| team_supply(chunk).unwrap().score())
        .sum::<u32>())
}

fn team_supply(input: &[Input]) -> Option<Supplies> {
    assert_eq!(input.len(), 3);
    for s1 in input[0].supplies.iter() {
        for s2 in input[1].supplies.iter() {
            for s3 in input[2].supplies.iter() {
                if s1 == s2 && s2 == s3 {
                    return Some(*s1);
                }
            }
        }
    }
    None
}

struct Input {
    supplies: Vec<Supplies>,
}

impl Input {
    fn parse(input: &str) -> Result<Vec<Self>> {
        input
            .lines()
            .map(Input::from_line)
            .collect::<Result<Vec<_>>>()
    }

    fn from_line(line: &str) -> Result<Self> {
        let supplies = line
            .trim()
            .chars()
            .map(Supplies::from_char)
            .collect::<Option<Vec<_>>>()
            .context("could not parse input")?;
        Ok(Self { supplies })
    }

    fn find_match_1(&self) -> Option<Supplies> {
        assert!(self.supplies.len() % 2 == 0);
        let compartment_len = self.supplies.len() / 2;
        // probably quicker to sort
        for ch in self.supplies[..compartment_len].iter() {
            for ch2 in self.supplies[compartment_len..].iter() {
                if ch == ch2 {
                    return Some(*ch);
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Supplies {
    ch: char,
}

impl Supplies {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'a'..='z' | 'A'..='Z' => Some(Self { ch }),
            _ => None,
        }
    }

    fn score(self) -> u32 {
        match self.ch {
            'a'..='z' => self.ch as u32 - 'a' as u32 + 1,
            'A'..='Z' => self.ch as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    }
}
