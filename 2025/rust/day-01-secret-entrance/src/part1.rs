use std::ops::Range;
use std::str::FromStr;

use day_01_secret_entrance::{Direction, MAX, Rotation};

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../input/part-1.txt");
    let steps = input.lines().collect::<Vec<_>>();

    let pw = get_password(50, steps)?;

    println!("{}", pw);

    Ok(())
}

fn get_password(start: usize, steps: Vec<&str>) -> anyhow::Result<usize> {
    let mut count = 0;
    let mut at = start;

    for s in steps {
        at = move_to(at, Rotation::from_str(s)?);
        if at == 0 {
            count += 1
        }
    }

    Ok(count)
}

fn move_to(at: usize, rotation: Rotation) -> usize {
    let bound = MAX + 1;
    let Rotation { direction, step } = rotation;

    match direction {
        Direction::Right => (at + step) % bound,
        Direction::Left => (at + bound - (step % bound)) % bound,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(50, vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"], 3)]
    fn test_get_password(#[case] start: usize, #[case] steps: Vec<&str>, #[case] expected: usize) {
        let to = get_password(start, steps);
        assert!(to.is_ok());
        assert_eq!(to.unwrap(), expected);
    }

    #[rstest]
    #[case(11, "R8", 19)]
    #[case(19, "L19", 0)]
    #[case(0, "L1", 99)]
    #[case(99, "R1", 0)]
    #[case(5, "L10", 95)]
    #[case(95, "R5", 0)]
    #[case(50, "L68", 82)]
    #[case(82, "L30", 52)]
    #[case(52, "R48", 0)]
    #[case(0, "L5", 95)]
    #[case(95, "R60", 55)]
    #[case(55, "L55", 0)]
    #[case(0, "L1", 99)]
    #[case(99, "L99", 0)]
    #[case(0, "R14", 14)]
    #[case(14, "L82", 32)]
    fn test_move_to(#[case] at: usize, #[case] rotation: &str, #[case] expected: usize) {
        let r = Rotation::from_str(rotation).unwrap();
        let des = move_to(at, r);

        assert_eq!(des, expected);
    }
}
