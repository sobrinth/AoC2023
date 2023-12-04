use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .fold(vec![], |mut matches, line| {
            let nums = line.split_once(':').unwrap().1;
            let (winning, guess) = nums.split_once('|').unwrap();
            let guesses: HashSet<usize> =
                guess.split_whitespace().flat_map(|g| g.parse()).collect();
            let winners: HashSet<usize> =
                winning.split_whitespace().flat_map(|g| g.parse()).collect();
            matches.push(guesses.intersection(&winners).count());
            matches
        })
        .iter()
        .map(|&matches| if matches > 0 { 1 << (matches - 1) } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let result = input.lines().fold(vec![], |mut matches, line| {
        let (_, nums) = line.split_once(':').unwrap();
        let (winning, guess) = nums.split_once('|').unwrap();
        let guesses: HashSet<usize> = guess.split_whitespace().flat_map(|g| g.parse()).collect();
        let winners: HashSet<usize> = winning.split_whitespace().flat_map(|g| g.parse()).collect();
        matches.push(guesses.intersection(&winners).count());
        matches
    });
    (0..result.len()).map(|x| recurse(&result, x)).sum()
}

fn recurse(input: &Vec<usize>, cur_idx: usize) -> usize {
    let cur_item = input[cur_idx];

    if cur_item == 0 {
        1
    } else {
        // recurse for the number of 'matches' in the current item and hope we stop at some point :)
        1 + (cur_idx..(cur_idx + cur_item))
            .map(|i| recurse(input, i + 1))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13)
    }

    #[test]
    fn part_two() {
        let output = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(output, 30)
    }
}
