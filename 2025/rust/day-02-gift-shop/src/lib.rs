use std::ops::RangeInclusive;

use anyhow::Context;

pub fn sum_invalid_ids<F>(input: &str, checker: F) -> anyhow::Result<usize>
where
    F: Fn(usize) -> bool + Copy,
{
    let mut total = 0;
    for s in input.split(',') {
        let count = find_invalid_ids(s, checker)?.sum::<usize>();
        total += count;
    }
    Ok(total)
}

pub fn find_invalid_ids<F>(input: &str, checker: F) -> anyhow::Result<impl Iterator<Item = usize>>
where
    F: Fn(usize) -> bool,
{
    let range = parse_range(input)?;
    Ok(range.filter(move |id| checker(*id)))
}

pub fn parse_range(value: &str) -> anyhow::Result<RangeInclusive<usize>> {
    let (start, end) = value.split_once('-').context("No delimeter")?;

    let start = start.parse::<usize>()?;
    let end = end.parse::<usize>()?;

    Ok(start..=end)
}
