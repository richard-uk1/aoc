#![feature(array_windows)]
use qu::ick_use::*;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(clap::Parser)]
struct Opt {
    day: u8,
    part: u8,
}

#[qu::ick]
fn main(opt: Opt) -> Result {
    if opt.part != 1 && opt.part != 2 {
        bail!("must specify part 1 or part 2");
    }
    let second = opt.part == 2;
    match (opt.day, second) {
        (1, false) => println!("{}", day1::first()?),
        (1, true) => println!("{}", day1::second()?),
        (2, false) => println!("{}", day2::first()?),
        (2, true) => println!("{}", day2::second()?),
        (3, false) => println!("{}", day3::first()?),
        (3, true) => println!("{}", day3::second()?),
        (4, false) => println!("{}", day4::first()?),
        (4, true) => println!("{}", day4::second()?),
        (5, false) => println!("{}", day5::first()?),
        (5, true) => println!("{}", day5::second()?),
        (6, false) => println!("{}", day6::first()?),
        (6, true) => println!("{}", day6::second()?),
        (7, false) => println!("{}", day7::first()?),
        (7, true) => println!("{}", day7::second()?),
        (8, false) => println!("{}", day8::first()?),
        (8, true) => println!("{}", day8::second()?),
        (9, false) => println!("{}", day9::first()?),
        (9, true) => println!("{}", day9::second()?),
        (10, false) => println!("{}", day10::first()?),
        (10, true) => println!("{}", day10::second()?),
        (11, false) => println!("{}", day11::first()?),
        (11, true) => println!("{}", day11::second()?),
        (n, _) => bail!("day {} not yet implemented/out of bounds", n),
    }
    Ok(())
}
