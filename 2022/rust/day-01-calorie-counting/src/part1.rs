use itertools::Itertools;

const INPUT: &str = include_str!("../input/part1.txt");

pub fn run() -> color_eyre::Result<()> {
    let v = INPUT
        .lines()
        .map(|n| n.parse::<u64>())
        .batching(|it| {
            let mut sum = 0;
            while let Some(Ok(n)) = it.next() {
                sum += n;
            }
            if sum == 0 { None } else { Some(sum) }
        })
        .max();

    dbg!(v);

    Ok(())
}
