pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|p| p[0] < p[1])
        .count()
        .try_into()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|w| w.iter().sum::<i32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|p| p[0] < p[1])
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", 1), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 7);
    }
    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 5);
    }
}
