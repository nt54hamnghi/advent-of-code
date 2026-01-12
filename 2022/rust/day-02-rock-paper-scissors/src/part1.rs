use std::str::FromStr;

use color_eyre::eyre::{Ok, bail};
use itertools::Itertools;

const INPUT: &str = include_str!("../input/part-1.txt");

pub fn run() -> color_eyre::Result<()> {
    let total = INPUT
        .trim()
        .lines()
        .map(|l| l.parse::<Round>())
        .process_results(|it| it.map(|r| r.score()).sum::<usize>())?;

    println!("{total}");

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn inherent_points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(self, theirs: Self) -> bool {
        match (self, theirs) {
            ((Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)) => true,
            _ => false,
        }
    }

    fn outcome(self, theirs: Self) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => bail!("note a valid move: {value:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn inherent_points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    ours: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((theirs, ' ', ours)) = s.chars().collect_tuple() else {
            bail!("expected <theirs>SP<ours>EOF, got {s:?}")
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}
