fn solve_part_one(input: &str) -> i64 {
    input.lines().map(row_largest_joltage_part_one).sum()
}

fn solve_part_two(input: &str) -> i64 {
    input.lines().map(row_largest_joltage_part_two).sum()
}

fn row_largest_joltage_part_one(input: &str) -> i64 {
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

fn row_largest_joltage_part_two(input: &str) -> i64 {
    // highest starting digit is 9
    let mut joltage: i64 = 9;
    let mut digits = String::new();
    let mut remaining_digits = 12;
    let mut tokens = input.to_string();
    loop {
        dbg!((&input, &digits, &remaining_digits, &tokens, &joltage));
        let pattern = joltage.to_string();
        if let Some((_, rest)) = tokens.split_once(&pattern)
            && (remaining_digits == 1 || (!rest.is_empty()))
            && (remaining_digits == 1 || (rest.len() >= remaining_digits))
        {
            digits.push_str(&joltage.to_string());
            remaining_digits -= 1;
            joltage = 9; // reset joltage
            tokens = rest.to_string(); // only use rest of tokens
            if remaining_digits == 0 {
                return digits.parse::<i64>().unwrap();
            }
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
        assert_eq!(row_largest_joltage_part_one("987654321111111"), 98);
        assert_eq!(row_largest_joltage_part_one("811111111111119"), 89);
        assert_eq!(row_largest_joltage_part_one("234234234234278"), 78);
        assert_eq!(row_largest_joltage_part_one("818181911112111"), 92);
        assert_eq!(solve_part_one(&input), 357);
    }

    #[test]
    fn test_example_part_two() {
        let input = std::fs::read_to_string("inputs/example03.txt").unwrap();
        assert_eq!(
            row_largest_joltage_part_two("987654321111111"),
            987654321111
        );
        assert_eq!(
            row_largest_joltage_part_two("811111111111119"),
            811111111119
        );
        assert_eq!(
            row_largest_joltage_part_two("234234234234278"),
            434234234278
        );
        assert_eq!(
            row_largest_joltage_part_two("818181911112111"),
            888911112111
        );
        assert_eq!(solve_part_one(&input), 357);
    }
}
