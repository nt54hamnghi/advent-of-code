use std::ops::Range;
use std::str::FromStr;

const MIN: isize = 0;
const MAX: isize = 99;

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../input/part-1.txt");
    let steps = input.lines().collect::<Vec<_>>();

    let pw = get_password(50, steps)?;

    println!("{}", pw);

    Ok(())
}

fn get_password(start: isize, steps: Vec<&str>) -> anyhow::Result<isize> {
    let mut count = 0;
    let mut at = start;

    for s in steps {
        at = move_to(at, s)?;
        if at == 0 {
            count += 1
        }
    }

    Ok(count)
}

fn move_to(at: isize, rotation: &str) -> anyhow::Result<isize> {
    let (direction, step) = parse_rotation(rotation)?;
    let bound = MAX + 1;
    let to = match direction {
        "R" => (step + at) % bound,
        "L" => (bound - step + at) % bound,
        _ => unreachable!(),
    };

    Ok(to)
}

fn parse_rotation(v: &str) -> anyhow::Result<(&str, isize)> {
    let (dir, step) = v.split_at(1);
    match dir {
        "R" | "L" => {
            let step = isize::from_str(step)?;
            Ok((dir, step))
        }
        _ => anyhow::bail!("Unknown direction, expect 'R' or 'L'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(50, vec!["L68","L30","R48","L5","R60","L55","L1","L99","R14","L82"], 3)]
    fn test_get_password(#[case] start: isize, #[case] steps: Vec<&str>, #[case] expected: isize) {
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
    fn test_move_to(#[case] at: isize, #[case] rotation: &str, #[case] expected: isize) {
        let to = move_to(at, rotation);
        assert!(to.is_ok());
        assert_eq!(to.unwrap(), expected);
    }

    #[rstest]
    #[case("R42", "R", 42)]
    #[case("L42", "L", 42)]
    fn test_parse_rotation(
        #[case] input: &str,
        #[case] expected_dir: &str,
        #[case] expected_step: isize,
    ) {
        let result = parse_rotation(input);
        assert!(result.is_ok());

        let (dir, step) = result.unwrap();
        assert_eq!(dir, expected_dir);
        assert_eq!(step, expected_step);
    }
}
