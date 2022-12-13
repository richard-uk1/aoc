use qu::ick_use::*;

mod day12;

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
        (12, false) => println!("{}", day12::first()?),
        (12, true) => println!("{}", day12::second()?),
        (n, _) => bail!("day {} not yet implemented/out of bounds", n),
    }
    Ok(())
}
