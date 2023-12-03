use std::collections::HashMap;
use std::str::Lines;

use aoc2023::day_input;

fn main() {
    let numbers = get_numbers(day_input::lines());
    let symbols = get_symbols(day_input::lines());

    let answer: u32 = numbers.iter().filter(
        |(pos, number)| {
            get_adjacent_positions(pos, number).iter().any(|p| symbols.contains_key(p))
        }).map(|(_, number)| number.value).sum();
    println!("{answer}");
}

fn get_numbers(lines: Lines) -> HashMap<Pos, Number> {
    let mut ret: HashMap<Pos, Number> = HashMap::new();
    for (y, line) in lines.enumerate() {
        let mut prev_nums: Vec<char> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                prev_nums.push(c);
            } else if !prev_nums.is_empty() {
                let value: u32 = String::from_iter(&prev_nums).parse().unwrap();
                ret.insert(Pos::from_usize(x - prev_nums.len(), y ), Number { value });
                prev_nums.clear();
            }
        }
        if !prev_nums.is_empty() {
            let value: u32 = String::from_iter(&prev_nums).parse().unwrap();
            ret.insert(Pos::from_usize(line.len() - prev_nums.len(), y), Number { value });
        }
    }
    ret
}

fn get_symbols(lines: Lines) -> HashMap<Pos, Symbol> {
    let mut ret: HashMap<Pos, Symbol> = HashMap::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                ret.insert( Pos::from_usize(x, y), Symbol { symbol: c });
            }
        }
    }
    ret
}

fn get_adjacent_positions(pos: &Pos, number: &Number) -> Vec<Pos> {
    let num_digits = number.value.to_string().len() as i32;
    let mut ret: Vec<Pos> = Vec::new();
    let min_x = pos.x - 1;
    let max_x = pos.x + num_digits;

    for y in pos.y - 1 ..= pos.y + 1 {
        for x in min_x ..= max_x {
            if y != pos.y || x == min_x || x == max_x {
                ret.push(Pos { x, y });
            }
        }
    }

    ret
}

struct Number {
    value: u32
}

struct Symbol {
    symbol: char
}

#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    fn from_usize(x: usize, y: usize) -> Pos {
        Pos {
            x: x as i32,
            y: y as i32
        }
    }
}
