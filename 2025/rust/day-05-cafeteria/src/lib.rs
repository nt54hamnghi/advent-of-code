use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

pub fn parse_input(input: &str) -> anyhow::Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let ranges = input
        .lines()
        .take_while(|s| s.contains("-"))
        .into_iter()
        .map(parse_id_range)
        .collect::<Result<Vec<_>, _>>()?;

    let ids = input
        .lines()
        .skip_while(|s| s.contains("-") || s.is_empty())
        .into_iter()
        .map(usize::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((ranges, ids))
}

pub fn parse_id_range(s: &str) -> anyhow::Result<RangeInclusive<usize>> {
    let parsed = s
        .split_once("-")
        .map(|(s, e)| (s.parse::<usize>(), e.parse::<usize>()))
        .context("missing delimiter '-'")?;

    let s = parsed.0?;
    let e = parsed.1?;

    Ok(s..=e)
}
