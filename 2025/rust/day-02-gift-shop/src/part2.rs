use std::ops::RangeInclusive;

use anyhow::Context;
use day_02_gift_shop::sum_invalid_ids;

const INPUT: &str = include_str!("../input/part-2.txt");

pub fn run() -> anyhow::Result<()> {
    let result = sum_invalid_ids(INPUT, check_invalid_id)?;
    dbg!(result);

    Ok(())
}

pub fn check_invalid_id(id: usize) -> bool {
    let chars = id.to_string().chars().collect::<Vec<_>>();
    let div = find_divisors(chars.len());

    for d in div {
        if d == chars.len() {
            continue;
        }

        let mut chunks = chars.chunks(d);
        let first = chunks.next().expect("Chunks with no item");

        if chunks.all(|e| e == first) {
            return true;
        }
    }

    false
}

#[allow(clippy::manual_is_multiple_of)]
pub fn find_divisors(n: usize) -> Vec<usize> {
    let limit = n.isqrt();

    let mut d1 = Vec::new();
    let mut d2 = Vec::new();

    for i in 1..=limit {
        if n % i == 0 {
            d1.push(i);
            if i != n / i {
                d2.push(n / i);
            }
        }
    }

    d2.reverse();
    d1.extend(d2);

    d1
}

#[cfg(test)]
mod tests {
    use day_02_gift_shop::find_invalid_ids;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, vec![1])]
    #[case(2, vec![1, 2])]
    #[case(7, vec![1, 7])]
    #[case(12, vec![1, 2, 3, 4, 6, 12])]
    #[case(20, vec![1, 2, 4, 5, 10, 20])]
    #[case(100, vec![1, 2, 4, 5, 10, 20, 25, 50, 100])]
    fn test_find_divisors(#[case] n: usize, #[case] expected: Vec<usize>) {
        let result = find_divisors(n);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(12341234, true)]
    #[case(123123123, true)]
    #[case(1212121212, true)]
    #[case(1111111, true)]
    //
    #[case(11, true)]
    #[case(22, true)]
    #[case(99, true)]
    #[case(111, true)]
    #[case(999, true)]
    #[case(1010, true)]
    #[case(1188511885, true)]
    #[case(222222, true)]
    #[case(446446, true)]
    #[case(38593859, true)]
    #[case(565656, true)]
    #[case(824824824, true)]
    #[case(2121212121, true)]
    //
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
    #[case("95-115", vec![99, 111])]
    #[case("998-1012", vec![999, 1010])]
    #[case("1188511880-1188511890", vec![1188511885])]
    #[case("222220-222224", vec![222222])]
    #[case("1698522-1698528", vec![])]
    #[case("446443-446449", vec![446446])]
    #[case("38593856-38593862", vec![38593859])]
    #[case("565653-565659", vec![565656])]
    #[case("824824821-824824827", vec![824824824])]
    #[case("2121212118-2121212124", vec![2121212121])]
    fn test_check_id_ranges(#[case] input: &str, #[case] expected: Vec<usize>) {
        let result = find_invalid_ids(input, check_invalid_id)
            .unwrap()
            .collect::<Vec<_>>();
        assert_eq!(result, expected);
    }
}
