use std::fs;
use std::path::PathBuf;

/// Reads input file for a given day
/// Looks for dayXX/input.txt from workspace root
pub fn read_input(day: u8) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to workspace root
    path.push(format!("day{:02}", day));
    path.push("input.txt");

    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {:?}", path))
}

pub fn read_test_input(day: u8) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up to workspace root
    path.push(format!("day{:02}", day));
    path.push("test.txt");
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read test input file: {:?}", path))
}

/// Parse lines into a vector of parsed items
pub fn parse_lines<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}

/// Split input on blank lines (useful for grouped input)
pub fn split_groups(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "1\n2\n3";
        let nums: Vec<i32> = parse_lines(input);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_groups() {
        let input = "a\nb\n\nc\nd";
        let groups = split_groups(input);
        assert_eq!(groups, vec!["a\nb", "c\nd"]);
    }
}
