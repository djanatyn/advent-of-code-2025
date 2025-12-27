fn solve_part_one(input: &str) -> i64 {
    input.trim().split(",").flat_map(find_invalid_ids).sum()
}

fn solve_part_two(input: &str) -> i64 {
    input
        .trim()
        .split(",")
        .flat_map(find_invalid_ids_part_two)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/input02.txt").unwrap();
    println!("part 1: {}", solve_part_one(&input));
    println!("part 2: {}", solve_part_two(&input));
}

fn find_invalid_ids(input: &str) -> Vec<i64> {
    let range = match &input.split('-').collect::<Vec<&str>>()[..] {
        [begin, end] => {
            (dbg!(begin).parse::<i64>().unwrap())..(dbg!(end).parse::<i64>().unwrap() + 1)
        }
        _ => panic!("invalid range"),
    };
    dbg!(&range);
    let mut invalid_ids = vec![];
    for num in range {
        if invalid(&num.to_string()) {
            invalid_ids.push(num)
        }
    }
    dbg!(invalid_ids)
}

fn find_invalid_ids_part_two(input: &str) -> Vec<i64> {
    let range = match &input.split('-').collect::<Vec<&str>>()[..] {
        [begin, end] => (begin.parse::<i64>().unwrap())..(end.parse::<i64>().unwrap() + 1),
        _ => panic!("invalid range"),
    };
    let mut invalid_ids = vec![];
    for num in range {
        if invalid_part_two(&num.to_string()) {
            invalid_ids.push(num)
        }
    }
    invalid_ids
}

/// An id is invalid if it is made only of some sequence of digits repeated
/// twice.
fn invalid(input: &str) -> bool {
    let length = input.chars().count();
    // all odd IDs are valid
    if length % 2 == 1 {
        false
    } else {
        let (first, second) = input.split_at(length / 2);
        first == second
    }
}

fn invalid_part_two(input: &str) -> bool {
    let mut size = 1; // check if first character is repeated subsequence
    while size <= (input.len() / 2) {
        // split into byte chunks
        let mut chunks = input.as_bytes().chunks(size);
        // if all elements are equal, then the sequence is invalid
        let first = chunks.next().unwrap();
        if chunks.all(|next| next == first) {
            return true;
        } else {
            size += 1;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid() {
        assert!(invalid("11"));
        assert!(invalid("22"));
        assert!(invalid("1010"));
        assert!(invalid("1188511885"));
        assert!(invalid("222222"));
        assert!(invalid("446446"));
        assert!(invalid("38593859"));
        assert!(!(invalid("95")));
        assert!(!(invalid("115")));
        assert!(!(invalid("111")));
    }

    #[test]
    /// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    /// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    /// 824824821-824824827,2121212118-2121212124
    ///
    /// 11-22 has two invalid IDs, 11 and 22.
    /// 95-115 has one invalid ID, 99.
    /// 998-1012 has one invalid ID, 1010.
    /// 1188511880-1188511890 has one invalid ID, 1188511885.
    /// 222220-222224 has one invalid ID, 222222.
    /// 1698522-1698528 contains no invalid IDs.
    /// 446443-446449 has one invalid ID, 446446.
    /// 38593856-38593862 has one invalid ID, 38593859.
    ///
    /// The rest of the ranges contain no invalid IDs.
    fn test_example_part_one() {
        assert_eq!(find_invalid_ids("11-22"), vec![11, 22]);
        assert_eq!(find_invalid_ids("95-115"), vec![99]);
        assert_eq!(find_invalid_ids("998-1012"), vec![1010]);
        assert_eq!(find_invalid_ids("1188511880-1188511890"), vec![1188511885]);
        assert_eq!(find_invalid_ids("222220-222224"), vec![222222]);
        assert_eq!(find_invalid_ids("1698522-1698528"), vec![]);
        assert_eq!(find_invalid_ids("446443-446449"), vec![446446]);
        assert_eq!(find_invalid_ids("38593856-38593862"), vec![38593859]);

        let all_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve_part_one(all_ranges), 1227775554)
    }

    #[test]
    fn test_example_part_two() {
        assert_eq!(find_invalid_ids_part_two("11-22"), vec![11, 22]);
        assert_eq!(find_invalid_ids_part_two("95-115"), vec![99, 111]);
        assert_eq!(find_invalid_ids_part_two("998-1012"), vec![999, 1010]);
        assert_eq!(
            find_invalid_ids_part_two("1188511880-1188511890"),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids_part_two("222220-222224"), vec![222222]);
        assert_eq!(find_invalid_ids_part_two("1698522-1698528"), vec![]);
        assert_eq!(find_invalid_ids_part_two("446443-446449"), vec![446446]);
        assert_eq!(
            find_invalid_ids_part_two("38593856-38593862"),
            vec![38593859]
        );
        assert_eq!(find_invalid_ids_part_two("565653-565659"), vec![565656]);
        assert_eq!(
            find_invalid_ids_part_two("824824821-824824827"),
            vec![824824824]
        );
        assert_eq!(
            find_invalid_ids_part_two("2121212118-2121212124"),
            vec![2121212121]
        );

        let all_ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve_part_two(all_ranges), 4174379265)
    }
}
