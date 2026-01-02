use std::str::FromStr;

use day_01_secret_entrance::{Direction, MAX, Rotation};

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../input/part-2.txt");
    let steps = input.lines().collect::<Vec<_>>();

    let pw = get_password(50, steps)?;

    println!("{}", pw);

    Ok(())
}

fn get_password(start: usize, steps: Vec<&str>) -> anyhow::Result<usize> {
    let mut count = 0;
    let mut at = start;

    for s in steps {
        let (des, rounds) = move_to(at, Rotation::from_str(s)?);
        count += rounds;
        at = des;
        if at == 0 {
            count += 1
        }
    }

    Ok(count)
}

fn move_to(at: usize, rotation: Rotation) -> (usize, usize) {
    let bound = MAX + 1;
    let Rotation { direction, step } = rotation;

    match direction {
        Direction::Right => {
            let des = (at + step) % bound;

            let mut rounds = (at + step) / bound;
            if des == 0 {
                rounds -= 1;
            }

            (des, rounds)
        }
        Direction::Left => {
            let des = (at + bound - (step % bound)) % bound;

            let mut rounds = (des + step) / bound;
            if at == 0 {
                rounds -= 1;
            }

            (des, rounds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(50, "L68", 82, 1)]
    #[case(82, "L30", 52, 0)]
    #[case(52, "R48", 0, 0)]
    #[case(0, "L5", 95, 0)]
    #[case(95, "R60", 55, 1)]
    #[case(55, "L55", 0, 0)]
    #[case(0, "L1", 99, 0)]
    #[case(99, "L99", 0, 0)]
    #[case(0, "R14", 14, 0)]
    #[case(14, "L82", 32, 1)]
    #[case(50, "R1000", 50, 10)]
    #[case(50, "L1000", 50, 10)]
    fn test_count_zero_rounds(
        #[case] at: usize,
        #[case] rotation: &str,
        #[case] des: usize,
        #[case] expected: usize,
    ) {
        let rotation = Rotation::from_str(rotation).unwrap();
        let result = move_to(at, rotation);

        assert_eq!(result.1, expected)
    }
}
