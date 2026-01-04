use std::collections::{HashMap, HashSet};

use day_03_lobby::parse_power_bank_str;
use itertools::Itertools;
const INPUT: &str = include_str!("../input/part-2.txt");

pub fn run() -> anyhow::Result<()> {
    let v = INPUT
        .lines()
        .map(parse_power_bank_str)
        .map(|rs| rs.map(|b| power_on(&b, 12)))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<u64>();

    println!("{v}");

    Ok(())
}

fn power_on(bank: &[u32], count: usize) -> u64 {
    assert!(!bank.is_empty());
    assert!(count <= bank.len());

    let mut v = Vec::with_capacity(count);
    let mut idx = 0;
    let mut limit = count;

    while v.len() < count {
        let mut max_idx = idx;
        let mut max = bank[max_idx];
        while idx + limit <= bank.len() {
            if bank[idx] > max {
                max = bank[idx];
                max_idx = idx;
            }
            idx += 1;
        }
        v.push(max);
        limit -= 1;
        idx = max_idx + 1;
    }

    from_digits(&v)
}

fn from_digits(slc: &[u32]) -> u64 {
    let len = slc.len();
    let mut total = 0;
    for (i, v) in slc.iter().enumerate() {
        let exp = (len - i - 1).try_into().unwrap();
        let num = 10u64.pow(exp) * (*v as u64);
        total += num;
    }
    total
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_power_on(#[case] input: &str, #[case] expected: u64) {
        let bank = parse_power_bank_str(input).unwrap();
        let power = power_on(&bank, 12);
        assert_eq!(power, expected);
    }
}
