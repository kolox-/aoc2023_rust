fn main() {
    let input = include_str!("../../inputs/3.txt");
    let p1_output = part1(input);
    dbg!(p1_output);

    let p2_output = part2(input);
    dbg!(p2_output);
}

fn part1(input: &str) -> u64 {
    0
}

fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 8);
    }
}
