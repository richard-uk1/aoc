use qu::ick_use::*;

const INPUT: &str = include_str!("../input/1");

pub fn first() -> Result<u32> {
    let mut max = 0;
    let mut total = 0;
    for line in INPUT.lines() {
        if line.is_empty() {
            if total > max {
                max = total;
            }
            total = 0;
            continue;
        }
        let calories = line.parse::<u32>()?;
        total += calories;
    }
    if total > max {
        max = total;
    }
    Ok(max)
}

pub fn second() -> Result<u32> {
    let mut elves = parse(INPUT).map(sum).collect::<Result<Vec<_>>>()?;
    elves.sort();
    if elves.len() < 3 {
        bail!("not enough elves");
    }
    let len = elves.len();
    Ok(elves[len - 1] + elves[len - 2] + elves[len - 3])
}

fn sum(input: impl Iterator<Item = Result<u32>>) -> Result<u32> {
    let mut sum = 0;
    for i in input {
        sum += i?;
    }
    Ok(sum)
}

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = Result<u32>> + '_> + '_ {
    input.split("\n\n").map(|block| {
        block
            .lines()
            .map(|line| line.parse::<u32>().map_err(Into::into))
    })
}
