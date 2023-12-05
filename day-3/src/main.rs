use std::{collections::HashSet, str::FromStr};

use puzzle::RunPuzzleError;

#[derive(Debug)]
enum DayThreeError {
    BadPuzzleInput,
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    span_in_row: (usize, usize),
}

#[derive(Debug)]
struct Symbol {
    position_in_row: usize,
}

#[derive(Debug)]
struct Row {
    symbols: HashSet<usize>,
    parts: Vec<PartNumber>,
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
}

impl PartNumber {
    fn next_to_symbol_in_adjacent_row(&self, adjacent_row: &Row) -> bool {
        (self.span_in_row.0.saturating_sub(1)..=self.span_in_row.1 + 1)
            .any(|row_index| adjacent_row.symbols.get(&row_index).is_some())
    }
}

impl FromStr for Schematic {
    type Err = DayThreeError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: string
                .lines()
                .map(|line| line.parse::<Row>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromStr for Row {
    type Err = DayThreeError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut symbols = HashSet::new();
        let mut parts = Vec::new();
        while let Some(next_entry_start_index) = string.chars().skip(i).position(|c| c != '.') {
            let is_number = string
                .chars()
                .nth(i + next_entry_start_index)
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false);

            if is_number {
                let number_end_index = string
                    .chars()
                    .enumerate()
                    .skip(i + next_entry_start_index)
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .unwrap()
                    .0;
                let number = &string[i + next_entry_start_index..number_end_index + 1]
                    .parse::<u32>()
                    .map_err(|_| DayThreeError::BadPuzzleInput)?;
                parts.push(PartNumber {
                    value: *number,
                    span_in_row: (i + next_entry_start_index, number_end_index),
                });
                i = number_end_index + 1;
            } else {
                symbols.insert(next_entry_start_index + i);
                i += next_entry_start_index + 1;
            }
        }

        Ok(Self { symbols, parts })
    }
}

fn solve_part_one(input: String) -> Result<String, DayThreeError> {
    let schematic: Schematic = input.parse()?;
    let mut all_part_numbers = Vec::<u32>::new();

    for (row_i, row) in schematic.rows.iter().enumerate() {
        for part in &row.parts {
            // Check after the number in this row
            if row.symbols.get(&(part.span_in_row.1 + 1)).is_some() {
                all_part_numbers.push(part.value);
                continue;
            }
            // Check before the number in this row
            if row
                .symbols
                .get(&(part.span_in_row.0.saturating_sub(1)))
                .is_some()
            {
                all_part_numbers.push(part.value);
                continue;
            }
            // Check the row above
            if row_i > 0
                && schematic
                    .rows
                    .get(row_i - 1)
                    .map(|previous_row| part.next_to_symbol_in_adjacent_row(previous_row))
                    .unwrap_or(false)
            {
                all_part_numbers.push(part.value);
                continue;
            }
            // Check the row below
            if schematic
                .rows
                .get(row_i + 1)
                .map(|next_row| part.next_to_symbol_in_adjacent_row(next_row))
                .unwrap_or(false)
            {
                all_part_numbers.push(part.value);
                continue;
            }
        }
    }

    let count = all_part_numbers.iter().sum::<u32>();
    Ok(format!("{count}"))
}

fn solve_part_two(input: String) -> Result<String, DayThreeError> {
    /*
     *  Plan:
     *   - Build a hash map from "gear coordinate" to "adjacent numbers" using similar logic to
     *     part one:
     *     - Change the symbols to a hashmap from index to character
     *     - When we do the work to find adjacent symbols above, if the symbol is a gear, put it
     *       in the output hasmap and put the number in it's adjacent numbers list
     *     - filter map the list to get the gear ratios
     */
    todo!();
}

fn main() -> Result<(), RunPuzzleError<DayThreeError>> {
    let result = puzzle::run(solve_part_one)?;
    println!("part_one = {result}");

    let result = puzzle::run(solve_part_two)?;
    println!("part_two = {result}");

    Ok(())
}
