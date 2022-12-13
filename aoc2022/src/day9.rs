use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res, value},
    sequence::tuple,
    IResult,
};
use qu::ick_use::*;
use std::{collections::HashSet, fmt};
use Dir::*;

const INPUT: &str = include_str!("../input/9");

pub fn first() -> Result<usize> {
    let input = Inst::parse(INPUT)?;
    let mut sim = Simulation1::new();
    for inst in input {
        sim.step_many(inst.dir, inst.amt);
    }
    Ok(sim.end_pos_seen.len())
}

pub fn second() -> Result<usize> {
    let input = Inst::parse(INPUT)?;
    let mut sim = Simulation2::new();
    for inst in input {
        sim.step_many(inst.dir, inst.amt);
    }
    Ok(sim.end_pos_seen.len())
}

struct Simulation1 {
    iter: usize,
    start_pos: (isize, isize),
    end_pos: (isize, isize),
    end_pos_seen: HashSet<(isize, isize)>,
}

impl Simulation1 {
    fn new() -> Self {
        Self {
            iter: 0,
            start_pos: (0, 0),
            end_pos: (0, 0),
            end_pos_seen: HashSet::from([(0, 0)]),
        }
    }

    fn step(&mut self, dir: Dir) {
        self.start_pos = dir.move_point(self.start_pos);
        self.end_pos = step(self.start_pos, self.end_pos);
        self.end_pos_seen.insert(self.end_pos);
        self.iter += 1;
    }

    fn step_many(&mut self, dir: Dir, amt: usize) {
        for _ in 0..amt {
            self.step(dir);
        }
    }

    #[allow(dead_code)]
    fn draw_seen(&self) -> impl fmt::Display + '_ {
        struct DrawSeen<'a>(&'a Simulation1);

        impl fmt::Display for DrawSeen<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.0.end_pos_seen.is_empty() {
                    return Ok(());
                }
                let min_x = self.0.end_pos_seen.iter().map(|(x, _)| *x).min().unwrap();
                let max_x = self.0.end_pos_seen.iter().map(|(x, _)| *x).max().unwrap();
                let min_y = self.0.end_pos_seen.iter().map(|(_, y)| *y).min().unwrap();
                let max_y = self.0.end_pos_seen.iter().map(|(_, y)| *y).max().unwrap();
                for y_idx in min_y..=max_y {
                    for x_idx in min_x..=max_x {
                        if self.0.end_pos_seen.contains(&(x_idx, y_idx)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    writeln!(f)?;
                }
                Ok(())
            }
        }

        DrawSeen(self)
    }
}

struct Simulation2 {
    iter: usize,
    pos: [(isize, isize); 10],
    end_pos_seen: HashSet<(isize, isize)>,
}

impl Simulation2 {
    fn new() -> Self {
        Self {
            iter: 0,
            pos: [(0, 0); 10],
            end_pos_seen: HashSet::from([(0, 0)]),
        }
    }

    fn step(&mut self, dir: Dir) {
        self.pos[0] = dir.move_point(self.pos[0]);
        for i in 1..10 {
            self.pos[i] = step(self.pos[i - 1], self.pos[i]);
        }
        self.end_pos_seen.insert(self.pos[9]);
        self.iter += 1;
    }

    fn step_many(&mut self, dir: Dir, amt: usize) {
        for _ in 0..amt {
            self.step(dir);
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Right,
    Up,
    Left,
    Down,
}

impl Dir {
    fn move_point(self, p: (isize, isize)) -> (isize, isize) {
        match self {
            Left => (p.0 - 1, p.1),
            Right => (p.0 + 1, p.1),
            Up => (p.0, p.1 - 1),
            Down => (p.0, p.1 + 1),
        }
    }
}

#[derive(Debug)]
struct Inst {
    dir: Dir,
    amt: usize,
}

impl Inst {
    fn parse(i: &str) -> Result<Vec<Self>> {
        i.lines().map(Self::parse_line).collect()
    }

    fn parse_line(i: &str) -> Result<Inst> {
        let (_, inst) = parse_line(i).map_err(|_| format_err!("couldn't parse input"))?;
        Ok(inst)
    }
}

fn parse_line(i: &str) -> IResult<&str, Inst> {
    let dir_parser = alt((
        value(Left, tag("L")),
        value(Right, tag("R")),
        value(Up, tag("U")),
        value(Down, tag("D")),
    ));
    let val_parser = map_res(digit1, <usize as std::str::FromStr>::from_str);
    let (i, (dir, _, amt)) = all_consuming(tuple((dir_parser, tag(" "), val_parser)))(i)?;
    Ok((i, Inst { dir, amt }))
}

/// Assume that the start pos has moved this step. Calculate the end move.
///
/// Rules for each direciton:
///  - If it's the same it doesn't move
///  - If it's 1 away it moves only if the other direction is 2+ away
///  - If it's 2 away it always moves
fn step(leader: (isize, isize), us: (isize, isize)) -> (isize, isize) {
    let leader_x_diff = leader.0 - us.0;
    let leader_y_diff = leader.1 - us.1;

    let us_x_diff = match leader_x_diff {
        isize::MIN..=-2 => -1,
        -1 => {
            if leader_y_diff.abs() >= 2 {
                -1
            } else {
                0
            }
        }
        0 => 0,
        1 => {
            if leader_y_diff.abs() >= 2 {
                1
            } else {
                0
            }
        }
        2..=isize::MAX => 1,
        _ => unreachable!(),
    };
    let us_y_diff = match leader_y_diff {
        isize::MIN..=-2 => -1,
        -1 => {
            if leader_x_diff.abs() >= 2 {
                -1
            } else {
                0
            }
        }
        0 => 0,
        1 => {
            if leader_x_diff.abs() >= 2 {
                1
            } else {
                0
            }
        }
        2..=isize::MAX => 1,
        _ => unreachable!(),
    };
    (us.0 + us_x_diff, us.1 + us_y_diff)
}

#[test]
fn test_move_point() {
    assert_eq!(Up.move_point((0, 0)), ((0, -1)));
    assert_eq!(Up.move_point((1, 2)), ((1, 1)));
    assert_eq!(Left.move_point((0, 0)), ((-1, 0)));
    assert_eq!(Right.move_point((1, 2)), ((2, 2)));
}

#[test]
fn test_step() {
    assert_eq!(step((0, 0), (0, 0)), (0, 0));
    assert_eq!(step((1, 0), (0, 0)), (0, 0));
    assert_eq!(step((1, 1), (0, 0)), (0, 0));
    assert_eq!(step((2, 0), (0, 0)), (1, 0));
    assert_eq!(step((2, 1), (0, 0)), (1, 1));
    assert_eq!(step((2, 2), (0, 0)), (1, 1));
    assert_eq!(step((3, 2), (1, 1)), (2, 2));
    assert_eq!(step((-2, -1), (0, 0)), (-1, -1));
    assert_eq!(step((0, 0), (-2, -1)), (-1, 0));
    assert_eq!(step((62, 66), (62, 64)), (62, 65));
}
