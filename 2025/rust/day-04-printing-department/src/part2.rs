use day_04_printing_department::Grid;

const INPUT: &str = include_str!("../input/part-2.txt");

pub fn run() -> anyhow::Result<()> {
    let mut g = Grid::from(INPUT.trim());
    let mut total = 0;

    loop {
        let removed = remove_paper_rolls(&mut g);
        if removed.is_empty() {
            break;
        }
        total += removed.len();
    }

    println!("{total}");
    // println!("{g}");

    Ok(())
}

fn remove_paper_rolls(g: &mut Grid) -> Vec<usize> {
    let mut to_remove = Vec::new();

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
            to_remove.push(i);
        }
    }

    for i in to_remove.iter() {
        g.data[*i] = 'x'
    }

    to_remove
}
