pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|v| 10 * v.first().unwrap() + v.last().unwrap())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            // Kind of hacky replace solution to be independent of order of numbers in string...
            // example is eightwothree -> should be 823, but wihtout readding the text it would only find 2 in this case
            l.replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|v| 10 * v.first().unwrap() + v.last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142)
    }

    #[test]
    fn part_two() {
        let output = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(output, 281)
    }
}
