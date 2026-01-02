#![allow(clippy::unreadable_literal)]

use std::ops::RangeInclusive;

use anyhow::Context;

const INPUT: &str = include_str!("../input/part-1.txt");

pub fn run() -> anyhow::Result<()> {
    let result = sum_invalid_ids(INPUT)?;
    dbg!(result);

    Ok(())
}

pub fn sum_invalid_ids(input: &str) -> anyhow::Result<usize> {
    let mut total = 0;
    for s in input.split(',') {
        let count = find_invalid_ids(s)?.sum::<usize>();
        total += count;
    }
    Ok(total)
}

pub fn find_invalid_ids(input: &str) -> anyhow::Result<impl Iterator<Item = usize>> {
    let range = parse_range(input)?;
    Ok(range.filter(|id| check_invalid_id(*id)))
}

pub fn check_invalid_id(id: usize) -> bool {
    let id = id.to_string();
    if !id.len().is_multiple_of(2) {
        return false;
    }
    let (f, s) = id.split_at(id.len() / 2);
    f == s
}

pub fn parse_range(r: &str) -> anyhow::Result<RangeInclusive<usize>> {
    let (start, end) = r.split_once('-').context("No delimeter")?;

    let start = start.parse::<usize>()?;
    let end = end.parse::<usize>()?;

    Ok(start..=end)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(11, true)]
    #[case(22, true)]
    #[case(1010, true)]
    #[case(1188511885, true)]
    #[case(222222, true)]
    #[case(446446, true)]
    #[case(38593859, true)]
    #[case(12, false)]
    #[case(95, false)]
    #[case(998, false)]
    #[case(1188511880, false)]
    #[case(222220, false)]
    #[case(1698522, false)]
    #[case(446443, false)]
    fn test_check_id(#[case] id: usize, #[case] expected: bool) {
        assert_eq!(check_invalid_id(id), expected);
    }

    #[rstest]
    #[case("11-22", vec![11, 22])]
    #[case("95-115", vec![99])]
    #[case("998-1012", vec![1010])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    #[case("1698522-1698528", vec![])]
    #[case("446443-446449", vec![446446])]
    #[case("38593856-38593862", vec![38593859])]
    #[case("565653-565659", vec![])]
    #[case("824824821-824824827", vec![])]
    #[case("2121212118-2121212124", vec![])]
    fn test_check_id_ranges(#[case] input: &str, #[case] expected: Vec<usize>) {
        let result = find_invalid_ids(input).unwrap().collect::<Vec<_>>();
        assert_eq!(result, expected);
    }
}
