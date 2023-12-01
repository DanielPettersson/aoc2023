use aoc2023::day_input;

fn main() {
    let answer = day_input::lines().map(get_calibration_value).sum::<u32>();
    println!("{answer}");
}

fn get_calibration_value(line: &str) -> u32 {
    let first_digit = get_first_number(line, line.chars());
    let last_digit = get_first_number(line, line.chars().rev());
    first_digit * 10 + last_digit
}

fn get_first_number(line: &str, mut chars: impl Iterator<Item=char>) -> u32 {
    chars.find(|c| c.is_numeric())
        .map(|c| char::to_digit(c, 10).unwrap()).unwrap_or_else(|| panic!("No number found in line {line}"))
}