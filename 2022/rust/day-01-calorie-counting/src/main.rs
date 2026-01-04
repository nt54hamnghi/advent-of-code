mod part1;
mod part2;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    part2::run();

    Ok(())
}
