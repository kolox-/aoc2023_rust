use std::cmp;

fn main() {
    let input = include_str!("../../inputs/2.txt");
    let p1_output = part1(input);
    dbg!(p1_output);

    let p2_output = part2(input);
    dbg!(p2_output);
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn within(&self, constraints: &CubeSet) -> bool {
        !self.sets.iter().any(|set| {
            (set.red > constraints.red)
                || (set.green > constraints.green)
                || (set.blue > constraints.blue)
        })
    }

    fn min_set_power(&self) -> u32 {
        let mut min_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in self.sets.iter() {
            min_set.red = cmp::max(min_set.red, set.red);
            min_set.green = cmp::max(min_set.green, set.green);
            min_set.blue = cmp::max(min_set.blue, set.blue);
        }
        min_set.red * min_set.green * min_set.blue
    }
}

fn parse_set(set: &str) -> CubeSet {
    let mut cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for cube in set.split(',').map(|s| s.trim()) {
        let count = u32::from_str_radix(&cube[..cube.find(' ').unwrap()], 10).unwrap();
        if cube.contains("red") {
            cube_set.red = count;
        } else if cube.contains("green") {
            cube_set.green = count;
        } else if cube.contains("blue") {
            cube_set.blue = count;
        }
    }
    cube_set
}

fn line_to_game(input: &str) -> Game {
    let game_id_start = input.find(' ').unwrap() + 1;
    let game_id_end = input.find(':').unwrap();
    let id = u32::from_str_radix(&input[game_id_start..game_id_end], 10).unwrap();

    let mut game = Game {
        id: id,
        sets: Vec::new(),
    };
    for set in input[game_id_end + 1..].split(';').map(|s| s.trim()) {
        game.sets.push(parse_set(set));
    }
    game
}

fn part1(input: &str) -> u32 {
    let constraint = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .map(|line| line_to_game(line))
        .filter(|game| game.within(&constraint))
        .fold(0, |sum, game| sum + game.id);
    games
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line_to_game(line))
        .fold(0, |sum, game| sum + game.min_set_power())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }

    #[test]
    fn full_input() {
        let input = include_str!("../../inputs/2.txt");
        assert_eq!(part1(input), 2285);
        assert_eq!(part2(input), 77021);
    }
}
