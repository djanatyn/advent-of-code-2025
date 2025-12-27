fn solve_part_one(input: &str) -> i64 {
    input.lines().map(row_largest_joltage).sum()
}

fn solve_part_two(input: &str) -> i64 {
    todo!()
}

fn row_largest_joltage(input: &str) -> i64 {
    // highest starting digit is 9
    let mut joltage: i64 = 9;
    loop {
        let pattern = joltage.to_string();
        if let Some((_, rest)) = input.split_once(&pattern)
            && !rest.is_empty()
        {
            let second_digit = rest
                .chars()
                .map(|i| i.to_string().parse::<i64>().unwrap())
                .max()
                .unwrap();
            return (joltage * 10) + second_digit;
        } else if joltage <= 0 {
            panic!("failed")
        } else {
            joltage -= 1;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/input03.txt").unwrap();
    println!("part 1: {}", solve_part_one(&input));
    println!("part 2: {}", solve_part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let input = std::fs::read_to_string("inputs/example03.txt").unwrap();
        assert_eq!(row_largest_joltage("987654321111111"), 98);
        assert_eq!(row_largest_joltage("811111111111119"), 89);
        assert_eq!(row_largest_joltage("234234234234278"), 78);
        assert_eq!(row_largest_joltage("818181911112111"), 92);
        assert_eq!(solve_part_one(&input), 357);
    }

    // #[test]
    // fn test_example_part_two() {
    //     todo!()
    // }
}
