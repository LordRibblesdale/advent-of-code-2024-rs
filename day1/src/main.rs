use anyhow::{ensure, Context, Result};
use std::path::PathBuf;

fn process_input(path: &PathBuf) -> Result<(Vec<u32>, Vec<u32>)> {
    let input =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read {path:?}"))?;

    let lines = input.lines().collect::<Vec<_>>();

    let mut first_locations_list = Vec::with_capacity(input.len());
    let mut second_locations_list = Vec::with_capacity(input.len());

    for line in lines {
        let values = line.split_whitespace().collect::<Vec<&str>>();
        let (first_value, second_value) = (values[0].parse::<u32>()?, values[1].parse::<u32>()?);

        first_locations_list.push(first_value);
        second_locations_list.push(second_value);
    }

    Ok((first_locations_list, second_locations_list))
}

fn main() -> Result<()> {
    let (mut first_values, mut second_values) = process_input(&PathBuf::from("input.txt"))?;

    ensure!(first_values.len() == second_values.len());

    first_values.sort();
    second_values.sort();

    let total_distance: u64 = first_values
        .iter()
        .zip(second_values.iter())
        .fold(0u64, |sum, (first_value, second_value)| {
            sum + first_value.abs_diff(*second_value) as u64
        });

    println!("Total distance: {}", total_distance);

    let similarity_score = first_values.iter().fold(0u64, |sum, first_value| {
        let entries_in_second_values = second_values
            .iter()
            .filter(|&second_value| *second_value == *first_value)
            .count() as u32;

        sum + (*first_value * entries_in_second_values) as u64
    });
    
    println!("Similarity score: {}", similarity_score);

    Ok(())
}
