use std::str::FromStr;

pub const MIN: usize = 0;
pub const MAX: usize = 99;

pub struct Rotation {
    pub direction: Direction,
    pub step: usize,
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
pub enum Direction {
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
