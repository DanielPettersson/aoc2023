use aoc2023::day_input;

fn main() {
    let answer: u32 = day_input::lines().map(parse_game).map(|g| g.get_power()).sum();
    println!("{answer}");
}

fn parse_game(line: &str) -> Game {
    let split_game: Vec<&str> = line.split(':').collect();
    let split_draws: Vec<&str> = split_game.last().unwrap().split(';').collect();
    let mut draws: Vec<Draw> = Vec::with_capacity(split_draws.len());

    for draw_str in split_draws {
        let mut draw = Draw::default();
        for draw_color_str in draw_str.split(", ") {
            let split_draw_color_str: Vec<&str> = draw_color_str.trim().split(' ').collect();
            let number: u32 = split_draw_color_str.first().unwrap().parse().unwrap();

            match *split_draw_color_str.last().unwrap() {
                "red" => draw.red = number,
                "green" => draw.green = number,
                "blue" => draw.blue = number,
                col => panic!("unexpected color: {col}")
            };
        }
        draws.push(draw);
    }

    Game {
        draws,
    }
}

struct Game {
    draws: Vec<Draw>
}

impl Game {
    fn get_power(&self) -> u32 {
        let max_red = self.draws.iter().map(|d| d.red).max().unwrap_or(0);
        let max_green = self.draws.iter().map(|d| d.green).max().unwrap_or(0);
        let max_blue = self.draws.iter().map(|d| d.blue).max().unwrap_or(0);
        max_red * max_green * max_blue
    }
}

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32
}
