use puzzle::RunPuzzleError;

#[derive(Debug)]
enum DayOneError {
    BadPuzzleInput,
}

fn solve_part_one(input: String) -> Result<String, DayOneError> {
    let digits: Result<u32, DayOneError> = input
        .lines()
        .map(|line| -> Result<u32, DayOneError> {
            let digit_one = line.chars().map(|c| c.to_digit(10)).find(|c| c.is_some());
            let digit_one = digit_one.flatten().ok_or(DayOneError::BadPuzzleInput)?;

            let digit_two = line
                .chars()
                .rev()
                .map(|c| c.to_digit(10))
                .find(|c| c.is_some());
            let digit_two = digit_two.flatten().ok_or(DayOneError::BadPuzzleInput)?;

            let value = digit_one * 10 + digit_two;

            Ok(value)
        })
        .sum();

    digits.map(|d| format!("{d}"))
}

fn substr_to_digit(value: &str, starting_at: usize) -> Option<u32> {
    if value[starting_at..].starts_with("one") {
        return Some(1);
    };
    if value[starting_at..].starts_with("two") {
        return Some(2);
    };
    if value[starting_at..].starts_with("three") {
        return Some(3);
    };
    if value[starting_at..].starts_with("four") {
        return Some(4);
    };
    if value[starting_at..].starts_with("five") {
        return Some(5);
    };
    if value[starting_at..].starts_with("six") {
        return Some(6);
    };
    if value[starting_at..].starts_with("seven") {
        return Some(7);
    };
    if value[starting_at..].starts_with("eight") {
        return Some(8);
    };
    if value[starting_at..].starts_with("nine") {
        return Some(9);
    };
    None
}

fn decode_digit(line: &str) -> Result<u32, DayOneError> {
    let first = line
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if let Some(digit) = c.to_digit(10) {
                Some(digit)
            } else {
                substr_to_digit(line, i)
            }
        })
        .find(|e| e.is_some())
        .flatten()
        .ok_or(DayOneError::BadPuzzleInput)?;
    let second = line
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if let Some(digit) = c.to_digit(10) {
                Some(digit)
            } else {
                substr_to_digit(line, line.len() - i - 1)
            }
        })
        .find(|e| e.is_some())
        .flatten()
        .ok_or(DayOneError::BadPuzzleInput)?;
    Ok(first * 10 + second)
}

fn solve_part_two(input: String) -> Result<String, DayOneError> {
    input
        .lines()
        .map(decode_digit)
        .sum::<Result<u32, DayOneError>>()
        .map(|d| format!("{d}"))
}

fn main() -> Result<(), RunPuzzleError<DayOneError>> {
    let result = puzzle::run(solve_part_one)?;
    println!("part_one = {result}");

    let result = puzzle::run(solve_part_two)?;
    println!("part_two = {result}");

    Ok(())
}
