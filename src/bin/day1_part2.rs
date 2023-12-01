use aoc2023::day_input;

static NUM_STRINGS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let answer = day_input::lines().map(get_calibration_value).sum::<u32>();
    println!("{answer}");
}

fn get_calibration_value(line: &str) -> u32 {
    let first_digit = get_first_number(line);
    let last_digit = get_last_number(line);
    let num = first_digit * 10 + last_digit;
    println!("{line} {num}");
    num

}

fn get_first_number(line: &str) -> u32 {
    let (mut pos, mut number) = line.char_indices().find(|(_, c)| c.is_numeric())
        .map(|(i, c)| (i, char::to_digit(c, 10).unwrap()))
        .unwrap_or((0, 0));

    for (num_i, num_string) in NUM_STRINGS.iter().enumerate() {
        if let Some(num_pos) = line.find(num_string) {
            if num_pos <= pos {
                pos = num_pos;
                number = (num_i + 1) as u32;
            }
        }
    }

    number
}

fn get_last_number(line: &str) -> u32 {
    let (mut pos, mut number) = line.char_indices().rev().find(|(_, c)| c.is_numeric())
        .map(|(i, c)| (i, char::to_digit(c, 10).unwrap()))
        .unwrap_or((0, 0));

    for (num_i, num_string) in NUM_STRINGS.iter().enumerate() {
        if let Some(num_pos) = line.rfind(num_string) {
            if num_pos >= pos {
                pos = num_pos;
                number = (num_i + 1) as u32;
            }
        }
    }

    number
}