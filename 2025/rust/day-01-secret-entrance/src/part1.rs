use std::ops::Range;
use std::str::FromStr;

const MIN: usize = 0;
const MAX: usize = 99;

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
        at = move_to(at, Rotation::from_str(s)?)?;
        if at == 0 {
            count += 1
        }
    }

    Ok(count)
}

fn move_to(at: usize, rotation: Rotation) -> anyhow::Result<usize> {
    let bound = MAX + 1;
    let Rotation { direction, step } = rotation;

    let v = match direction {
        Direction::Right => (at + step) % bound,
        Direction::Left => (at + bound - (step % bound)) % bound,
    };

    Ok(v)
}

struct Rotation {
    direction: Direction,
    step: usize,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.split_at(1);
        let direction = Direction::from_str(r.0)?;
        let step = usize::from_str(r.1)?;

        Ok(Self { direction, step })
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => anyhow::bail!("Unknown direction"),
        }
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
        let to = move_to(at, r);
        assert!(to.is_ok());
        assert_eq!(to.unwrap(), expected);
    }

    #[rstest]
    #[case("R42", Direction::Right, 42)]
    #[case("L42", Direction::Left, 42)]
    fn test_parse_rotation(
        #[case] input: &str,
        #[case] expected_dir: Direction,
        #[case] expected_step: usize,
    ) {
        let result = Rotation::from_str(input);
        assert!(result.is_ok());

        let r = result.unwrap();
        assert_eq!(r.direction, expected_dir);
        assert_eq!(r.step, expected_step);
    }
}
