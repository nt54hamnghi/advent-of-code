use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::prelude;

use day_05_cafeteria::parse_input;
use itertools::Itertools;

const INPUT: &str = include_str!("../input/part-2.txt");

pub fn run() -> anyhow::Result<()> {
    let (mut ranges, _) = parse_input(INPUT.trim())?;
    ranges.sort_by_key(|r| *r.start());
    // println!("{}", ranges.len());

    let total = ranges
        .into_iter()
        .coalesce(|a, b| merge_ranges(&a, &b).ok_or((a, b)))
        .map(|r| r.end() - r.start() + 1)
        .sum::<usize>();

    println!("{total}");

    Ok(())
}

fn merge_ranges(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if !is_overloap(&a, &b) {
        return None;
    }

    let start = *a.start().min(b.start());
    let end = *a.end().max(b.end());

    Some(start..=end)
}

fn is_overloap(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    let (sa, ea) = (a.start(), a.end());
    let (sb, eb) = (b.start(), b.end());

    a.contains(sb) || a.contains(eb) || b.contains(sa) || b.contains(ea)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::overlapping_middle(1..=5, 3..=7, true)]
    #[case::adjacent_at_end(1..=5, 5..=10, true)]
    #[case::overlapping_at_start(1..=5, 1..=3, true)]
    #[case::overlapping_middle_reversed(3..=7, 1..=5, true)]
    #[case::b_contained_in_a(1..=10, 3..=7, true)]
    #[case::a_contained_in_b(3..=7, 1..=10, true)]
    #[case::non_intersecting_gap(1..=5, 6..=10, false)]
    #[case::non_intersecting_gap_reversed(6..=10, 1..=5, false)]
    #[case::non_intersecting_with_gap(1..=3, 5..=7, false)]
    fn test_is_overlap(
        #[case] a: RangeInclusive<usize>,
        #[case] b: RangeInclusive<usize>,
        #[case] expected: bool,
    ) {
        assert_eq!(is_overloap(&a, &b), expected);
    }

    #[rstest]
    #[case::overlapping_middle(1..=5, 3..=7, Some(1..=7))]
    #[case::adjacent_at_end(1..=5, 5..=10, Some(1..=10))]
    #[case::overlapping_at_start(1..=5, 1..=3, Some(1..=5))]
    #[case::overlapping_middle_reversed(3..=7, 1..=5, Some(1..=7))]
    #[case::b_contained_in_a(1..=10, 3..=7, Some(1..=10))]
    #[case::a_contained_in_b(3..=7, 1..=10, Some(1..=10))]
    #[case::non_intersecting_gap(1..=5, 6..=10, None)]
    #[case::non_intersecting_gap_reversed(6..=10, 1..=5, None)]
    #[case::non_intersecting_with_gap(1..=3, 5..=7, None)]
    fn test_merge_ranges(
        #[case] a: RangeInclusive<usize>,
        #[case] b: RangeInclusive<usize>,
        #[case] expected: Option<RangeInclusive<usize>>,
    ) {
        assert_eq!(merge_ranges(&a, &b), expected);
    }
}
