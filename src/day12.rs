use aoc_runner_derive::{aoc, aoc_generator};

fn combinations(items: &[u64], size: u64) -> u64 {
    let order = items.len() as u64;
    let min_size = items.iter().sum::<u64>() + order - 1;
    let Some(diff) = size.checked_sub(min_size) else { return 0 };
    let n = diff + 1;
    (n..n + order).product::<u64>() / (1..=order).product::<u64>()
}

#[aoc_generator(day12)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day12, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn should_combine_order_1() {
        assert_eq!(combinations(&[2], 1), 0); // Too small
        assert_eq!(combinations(&[2], 2), 1);
        assert_eq!(combinations(&[2], 3), 2);
        assert_eq!(combinations(&[2], 4), 3);
        assert_eq!(combinations(&[2], 5), 4);
    }

    #[test]
    fn should_combine_order_2() {
        assert_eq!(combinations(&[2, 1], 3), 0); // Too small
        assert_eq!(combinations(&[2, 1], 4), 1);
        assert_eq!(combinations(&[2, 1], 5), 3);
        assert_eq!(combinations(&[2, 1], 6), 6);
        assert_eq!(combinations(&[2, 1], 7), 10);
    }

    #[test]
    fn should_combine_order_3() {
        assert_eq!(combinations(&[2, 1, 1], 5), 0); // Too small
        assert_eq!(combinations(&[2, 1, 1], 6), 1);
        assert_eq!(combinations(&[2, 1, 1], 7), 4);
        assert_eq!(combinations(&[2, 1, 1], 8), 10);
        assert_eq!(combinations(&[2, 1, 1], 9), 20);
    }
}
