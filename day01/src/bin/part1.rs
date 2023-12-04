fn main() {
    let input = include_str!("../../../inputs/1.txt");
    let p1_output = part1(input);
    dbg!(p1_output);

    let p2_output = part2(input);
    dbg!(p2_output);
}

fn get_calibration(string: &str) -> u32 {
    let digits: Vec<u32> = string.chars().filter_map(|c| c.to_digit(10)).collect();
    digits[0] * 10 + digits[digits.len() - 1]
}

fn numerize(line: &str) -> String {
    line.replace("one", "o1ne")
        .replace("two", "t2wo")
        .replace("three", "t3hree")
        .replace("four", "f4our")
        .replace("five", "f5ive")
        .replace("six", "s6ix")
        .replace("seven", "s7even")
        .replace("eight", "e8ight")
        .replace("nine", "n9ine")
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| get_calibration(s))
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| numerize(s))
        .map(|s| get_calibration(&s))
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }

    #[test]
    fn day1_part2() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }

    #[test]
    fn day1_full_input() {
        let input = include_str!("../../../inputs/1.txt");
        assert_eq!(part1(input), 53386);
        assert_eq!(part2(input), 53312);
    }
}
