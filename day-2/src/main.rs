use std::str::FromStr;

use puzzle::RunPuzzleError;

#[derive(Debug)]
enum DayTwoError {
    BadPuzzleInput,
}

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: u8,
    blue: u8,
    green: u8,
}

impl Set {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

impl FromStr for Set {
    type Err = DayTwoError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        while i < string.len() {
            let count_end_index = string
                .chars()
                .skip(i)
                .position(|c| c == ' ')
                .ok_or(DayTwoError::BadPuzzleInput)?;
            let count = string[i..i + count_end_index]
                .parse::<u8>()
                .map_err(|_| DayTwoError::BadPuzzleInput)?;
            if string[i + count_end_index + 1..].starts_with("red") {
                red += count;
                i += count_end_index + 6;
                continue;
            }
            if string[i + count_end_index + 1..].starts_with("blue") {
                blue += count;
                i += count_end_index + 7;
                continue;
            }
            if string[i + count_end_index + 1..].starts_with("green") {
                green += count;
                i += count_end_index + 8;
                continue;
            }
        }

        Ok(Self { red, blue, green })
    }
}

impl FromStr for Game {
    type Err = DayTwoError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if !string.starts_with("Game ") {
            return Err(DayTwoError::BadPuzzleInput);
        };
        let mut index = 5;
        let game_id_end_index = string
            .chars()
            .skip(index)
            .position(|c| c == ':')
            .ok_or(DayTwoError::BadPuzzleInput)?;

        let id = string[index..index + game_id_end_index]
            .parse::<u8>()
            .map_err(|_| DayTwoError::BadPuzzleInput)?;

        index += game_id_end_index + 2;

        let mut sets = Vec::new();
        while let Some(semi_colon_position) = string.chars().skip(index).position(|c| c == ';') {
            sets.push(string[index..index + semi_colon_position].parse::<Set>()?);
            index += semi_colon_position + 2;
        }

        sets.push(string[index..].parse::<Set>()?);

        Ok(Self { id, sets })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        !self.sets.iter().any(|set| !set.is_valid())
    }

    fn minimum_set(self) -> Result<Set, DayTwoError> {
        self.sets
            .into_iter()
            .reduce(|mut acc, s| {
                acc.red = std::cmp::max(acc.red, s.red);
                acc.blue = std::cmp::max(acc.blue, s.blue);
                acc.green = std::cmp::max(acc.green, s.green);
                acc
            })
            .ok_or(DayTwoError::BadPuzzleInput)
    }
}

fn solve_part_one(input: String) -> Result<String, DayTwoError> {
    let count = input
        .lines()
        .map(|l| l.parse::<Game>())
        .filter(|r| r.as_ref().map(|g| g.is_valid()).unwrap_or(true))
        .map(|r| r.map(|g| g.id as u32))
        .sum::<Result<u32, DayTwoError>>()?;
    Ok(format!("{count}"))
}

fn solve_part_two(input: String) -> Result<String, DayTwoError> {
    let count = input
        .lines()
        .map(|l| {
            let game = l.parse::<Game>()?;
            game.minimum_set().map(|s| s.power())
        })
        .sum::<Result<u32, DayTwoError>>()?;
    Ok(format!("{count}"))
}

fn main() -> Result<(), RunPuzzleError<DayTwoError>> {
    let result = puzzle::run(solve_part_one)?;
    println!("part_one = {result}");

    let result = puzzle::run(solve_part_two)?;
    println!("part_two = {result}");

    Ok(())
}
