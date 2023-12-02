use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let limit = (12, 13, 14); // r, g, b
    input
        .lines()
        .map(|line| {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            line.split_once(':')
                .expect("Error with line")
                .1
                // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                .split(';')
                .map(|pulls| {
                    // 3 blue, 4 red
                    pulls.split(',').fold((0, 0, 0), |mut balls, raw_pulls| {
                        // 3 blue
                        match raw_pulls.trim().split_once(' ') {
                            Some((num, "red")) => balls.0 += num.parse::<u32>().unwrap(),
                            Some((num, "green")) => balls.1 += num.parse::<u32>().unwrap(),
                            Some((num, "blue")) => balls.2 += num.parse::<u32>().unwrap(),
                            _ => unreachable!("Not a valid color"),
                        }
                        balls
                    })
                })
                .collect_vec()
        })
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|balls| balls.0 <= limit.0 && balls.1 <= limit.1 && balls.2 <= limit.2)
        })
        .map(|(id, _)| id as u32 + 1)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // Line has format: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            line.split_once(':')
                .expect("Error with line")
                .1
                .split(';')
                .map(|pulls| {
                    pulls.split(',').fold([0, 0, 0], |mut balls, raw_pulls| {
                        match raw_pulls.trim().split_once(' ') {
                            Some((num, "red")) => balls[0] += num.parse::<u32>().unwrap(),
                            Some((num, "green")) => balls[1] += num.parse::<u32>().unwrap(),
                            Some((num, "blue")) => balls[2] += num.parse::<u32>().unwrap(),
                            _ => unreachable!("Not a valid color"),
                        }
                        balls
                    })
                })
                .collect_vec()
        })
        .map(|game| {
            game
                .iter()
                .fold([0, 0, 0], |set, part| {
                    [
                        set[0].max(part[0]),
                        set[1].max(part[1]),
                        set[2].max(part[2]),
                    ]
                })
                .iter()
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8)
    }

    #[test]
    fn part_two() {
        let output = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(output, 2286)
    }
}
