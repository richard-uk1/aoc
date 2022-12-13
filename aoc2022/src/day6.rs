use qu::ick_use::*;

const INPUT: &str = include_str!("../input/6");

pub fn first() -> Result<u32> {
    for (idx, window) in INPUT.as_bytes().array_windows::<4>().enumerate() {
        let mut window = window.clone();
        window.sort();
        if window[0] < window[1] && window[1] < window[2] && window[2] < window[3] {
            return Ok((idx + 4).try_into().unwrap());
        }
    }
    bail!("couldn't find 4 non-matching characters")
}

pub fn second() -> Result<u32> {
    'windows: for (idx, window) in INPUT.as_bytes().array_windows::<14>().enumerate() {
        let mut window = window.clone();
        window.sort();
        for i in 0..13 {
            if window[i] == window[i + 1] {
                continue 'windows;
            }
        }
        return Ok((idx + 14).try_into().unwrap());
    }
    bail!("couldn't find 14 non-matching characters")
}
