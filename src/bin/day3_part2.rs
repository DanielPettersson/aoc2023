use std::collections::{HashMap, HashSet};
use std::str::Lines;

use aoc2023::day_input;

fn main() {
    let numbers = get_numbers(day_input::lines());
    let symbols = get_symbols(day_input::lines());

    let answer: u32 = symbols.iter().filter(|(_, s)| s.is_gear_symbol()).filter_map(|(p, _)| {
        let numbers = get_adjacent_positions(p).iter().filter_map(|p| numbers.get(p).map(|n| n.value)).collect::<HashSet<u32>>();
        if numbers.len() == 2 {
            Some(numbers.iter().product::<u32>())
        } else {
            None
        }
    }).sum();

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
                let num = Number { value };
                for xx in x - prev_nums.len()..x {
                    ret.insert(Pos::from_usize(xx, y), num);
                }
                prev_nums.clear();
            }
        }
        if !prev_nums.is_empty() {
            let value: u32 = String::from_iter(&prev_nums).parse().unwrap();
            let num = Number { value };
            for xx in line.len() - prev_nums.len()..line.len() {
                ret.insert(Pos::from_usize(xx, y), num);
            }
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
                ret.insert(Pos::from_usize(x, y), Symbol { symbol: c });
            }
        }
    }
    ret
}

fn get_adjacent_positions(pos: &Pos) -> Vec<Pos> {
    let mut ret: Vec<Pos> = Vec::new();
    for y in pos.y - 1..=pos.y + 1 {
        for x in pos.x - 1..=pos.x + 1 {
            if y != pos.y || x != pos.x {
                ret.push(Pos { x, y });
            }
        }
    }
    ret
}

#[derive(Copy, Clone)]
struct Number {
    value: u32,
}

struct Symbol {
    symbol: char,
}

impl Symbol {
    fn is_gear_symbol(&self) -> bool {
        self.symbol == '*'
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn from_usize(x: usize, y: usize) -> Pos {
        Pos {
            x: x as i32,
            y: y as i32,
        }
    }
}
