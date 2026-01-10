use std::fmt::{Debug, Display};
use std::mem::Discriminant;
use std::ptr::write;
use std::str::FromStr;

use day_04_printing_department::Grid;

const INPUT: &str = include_str!("../input/part-1.txt");

pub fn run() -> anyhow::Result<()> {
    let g = Grid::from(INPUT.trim());
    let mut check = g.data.clone();

    let mut total = 0;
    for i in 0..g.data.len() {
        if g.data[i] != '@' {
            continue;
        }

        let a = g
            .adjecent_of(i)
            .map(|v| g.data[v])
            .filter(|c| *c == '@')
            .count();

        if a < 4 {
            total += 1;
            check[i] = 'x';
        }
    }

    println!("{total}");
    // println!("{g}");

    Ok(())
}
