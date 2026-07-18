use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let input = utils::read_input(2);
    let sum_invalid_ids = get_invalid_ids_sum(&input, validate_id);
    let sum_invalid_ids_part2 = get_invalid_ids_sum(&input, validate_id_part2);

    println!("Day 02 - Part 01: {:?}", sum_invalid_ids);
    println!("Day 02 - Part 02: {:?}", sum_invalid_ids_part2);
}

fn get_invalid_ids_sum<F>(input: &str, validator: F) -> u64
where
    F: Fn(u64) -> bool,
{
    let ids_tuple: Vec<Vec<u64>> = input
        .split(',')
        .map(|part| {
            let mut ids = part.split('-');
            let first_id = ids.next().unwrap().parse::<u64>().unwrap();
            let second_id = ids.next().unwrap().parse::<u64>().unwrap();
            (first_id, second_id)
        })
        .map(|ids_tuple| generate_ids_range(ids_tuple))
        .map(|ids_range| {
            ids_range
                .into_iter()
                .filter_map(|id| {
                    if validator(id) == false {
                        return Some(id);
                    }

                    return None;
                })
                .collect()
        })
        .collect();

    ids_tuple.iter().flatten().sum::<u64>()
}

fn generate_ids_range(ids_tuple: (u64, u64)) -> Vec<u64> {
    (ids_tuple.0..=ids_tuple.1).collect()
}

fn validate_id(id: u64) -> bool {
    let id_string = id.to_string();
    let id_length = id.to_string().len();
    if id_length % 2 != 0 {
        return true;
    }

    let half_index = id_length / 2;
    let halfs = id_string.split_at(half_index);
    hash_string(halfs.0) != hash_string(halfs.1)
}

fn validate_id_part2(id: u64) -> bool {
    let id_string = id.to_string();
    let id_length = id.to_string().len();
    let half_size = id_length / 2;
    let mut chunk_size = half_size;

    while chunk_size > 0 {
        let remainder = id_length % chunk_size;
        if remainder != 0 {
            chunk_size -= 1;
            continue;
        }

        let chunks: Vec<u64> = id_string
            .chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|chunk| {
                let chunk_str: String = chunk.iter().collect();
                chunk_str.parse::<u64>().unwrap()
            })
            .collect();

        if all_equal(&chunks) {
            return false;
        }

        chunk_size -= 1;
    }

    return true;
}

fn all_equal(v: &[u64]) -> bool {
    match v.first() {
        Some(first) => v.iter().all(|x| x == first),
        None => true,
    }
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ids() {
        // 11-22: two invalid IDs, 11 and 22
        assert!(!validate_id(11));
        assert!(!validate_id(22));

        // 95-115: one invalid ID, 99
        assert!(!validate_id(99));

        // 998-1012: one invalid ID, 1010
        assert!(!validate_id(1010));

        // 1188511880-1188511890: one invalid ID, 1188511885
        assert!(!validate_id(1_188_511_885));

        // 222220-222224: one invalid ID, 222222
        assert!(!validate_id(222_222));

        // 446443-446449: one invalid ID, 446446
        assert!(!validate_id(446_446));

        // 38593856-38593862: one invalid ID, 38593859
        assert!(!validate_id(38_593_859));
    }

    #[test]
    fn test_valid_ids() {
        // 95-115: everything except 99 is valid
        assert!(validate_id(95));
        assert!(validate_id(100));
        assert!(validate_id(115));

        // 1698522-1698528: no invalid IDs at all
        assert!(validate_id(1_698_522));
        assert!(validate_id(1_698_523));
        assert!(validate_id(1_698_524));
        assert!(validate_id(1_698_525));
        assert!(validate_id(1_698_526));
        assert!(validate_id(1_698_527));
        assert!(validate_id(1_698_528));
    }

    #[test]
    fn test_sum_invalid_ids_example() {
        let input = utils::read_test_input(2);
        let sum_invalid_ids = get_invalid_ids_sum(&input, validate_id);

        assert_eq!(sum_invalid_ids, 1227775554);
    }

    #[test]
    fn test_invalid_ids_part2() {
        // 11-22: two invalid IDs, 11 and 22
        assert!(!validate_id_part2(11));
        assert!(!validate_id_part2(22));

        // 95-115: two invalid IDs, 99 and 111
        assert!(!validate_id_part2(99));
        assert!(!validate_id_part2(111));

        // 998-1012: two invalid IDs, 999 and 1010
        assert!(!validate_id_part2(999));
        assert!(!validate_id_part2(1010));

        // 1188511880-1188511890: one invalid ID, 1188511885
        assert!(!validate_id_part2(1_188_511_885));

        // 222220-222224: one invalid ID, 222222
        assert!(!validate_id_part2(222_222));

        // 446443-446449: one invalid ID, 446446
        assert!(!validate_id_part2(446_446));

        // 38593856-38593862: one invalid ID, 38593859
        assert!(!validate_id_part2(38_593_859));

        // 565653-565659: one invalid ID, 565656
        assert!(!validate_id_part2(565_656));

        // 824824821-824824827: one invalid ID, 824824824
        assert!(!validate_id_part2(824_824_824));

        // 2121212118-2121212124: one invalid ID, 2121212121
        assert!(!validate_id_part2(2_121_212_121));
    }

    #[test]
    fn test_valid_ids_part2() {
        // 1698522-1698528: no invalid IDs at all
        assert!(validate_id_part2(12));
        assert!(validate_id_part2(200));
        assert!(validate_id_part2(1_698_522));
        assert!(validate_id_part2(1_698_523));
        assert!(validate_id_part2(1_698_524));
        assert!(validate_id_part2(1_698_525));
        assert!(validate_id_part2(1_698_526));
        assert!(validate_id_part2(1_698_527));
        assert!(validate_id_part2(1_698_528));
    }
}
