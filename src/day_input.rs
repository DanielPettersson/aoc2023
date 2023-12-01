use std::str::Lines;

pub fn lines() -> Lines<'static> {
    include_str!("input.txt").lines()
}