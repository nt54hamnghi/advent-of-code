use std::str::FromStr;

use anyhow::Context;
use day_03_lobby::parse_power_bank_str;

const INPUT: &str = include_str!("../input/part-1.txt");

pub fn run() -> anyhow::Result<()> {
    let v: u32 = INPUT
        .lines()
        .map(parse_power_bank_str)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|v| power_on(v))
        .sum();

    dbg!(v);

    Ok(())
}

fn power_on(bank: &[u32]) -> u32 {
    if bank.len() < 2 {
        panic!("not enough batteries")
    }

    let (mut f, mut fi) = (0, 0);
    for (i, n) in bank.iter().enumerate() {
        if *n > f {
            (f, fi) = (*n, i);
        }
    }

    let rest = if fi == bank.len() - 1 {
        &bank[0..fi]
    } else {
        &bank[fi + 1..]
    };

    let mut s = 0;
    for n in rest.iter() {
        if *n > s {
            s = *n
        }
    }

    // println!("f={:?}\tfi={:?}", f, fi);
    // println!("s={:?}\tsi={:?}", s, si);

    if fi == bank.len() - 1 {
        s * 10 + f
    } else {
        f * 10 + s
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_power_on(#[case] input: &str, #[case] expected: u32) {
        let bank = parse_power_bank_str(input).unwrap();
        let power = power_on(&bank);
        assert_eq!(power, expected);
    }
}
