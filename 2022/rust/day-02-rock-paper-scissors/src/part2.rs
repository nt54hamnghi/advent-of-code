use std::str::FromStr;

use color_eyre::eyre::{Ok, bail};
use itertools::Itertools;

const INPUT: &str = include_str!("../input/part-2.txt");

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
    const ALL_MOVES: [Move; 3] = [Self::Rock, Self::Paper, Self::Scissors];

    fn inherent_points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .into_iter()
            .find(|m| m.beats(self))
            .expect("at least one move beats us")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .into_iter()
            .find(|m| self.beats(*m))
            .expect("at least one move beats us")
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn beats(self, theirs: Self) -> bool {
        match (self, theirs) {
            ((Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper)) => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => bail!("note a valid move: {value:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn matching_move(self, theirs: Move) -> Move {
        match self {
            Outcome::Loss => theirs.losing_move(),
            Outcome::Draw => theirs.drawing_move(),
            Outcome::Win => theirs.winning_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => bail!("note a valid outcome: {value:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    theirs: Move,
    outcome: Outcome,
}

impl Round {
    fn score(self) -> usize {
        self.outcome.inherent_points() + self.outcome.matching_move(self.theirs).inherent_points()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((theirs, ' ', outcome)) = s.chars().collect_tuple() else {
            bail!("expected <theirs>SP<ours>EOF, got {s:?}")
        };

        Ok(Self {
            theirs: theirs.try_into()?,
            outcome: outcome.try_into()?,
        })
    }
}
