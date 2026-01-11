use day_05_cafeteria::parse_input;

const INPUT: &str = include_str!("../input/part-1.txt");

pub fn run() -> anyhow::Result<()> {
    let (ranges, ids) = parse_input(INPUT.trim())?;

    // println!("{:?}", v.0);
    // println!("{:?}", v.1);

    let mut total = 0;
    for item in ids {
        let check = ranges.iter().any(|r| r.contains(&item));
        total += usize::from(check);
    }

    println!("{}", total);

    Ok(())
}
