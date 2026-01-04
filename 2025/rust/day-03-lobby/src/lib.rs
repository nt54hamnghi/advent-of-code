use anyhow::Context;

pub fn parse_power_bank_str(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .chars()
        .map(|c| c.to_digit(10).context("Not a valid decimal digit"))
        .collect::<Result<Vec<_>, _>>()
}
