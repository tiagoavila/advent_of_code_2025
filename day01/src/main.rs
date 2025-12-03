use std::result;

fn main() {
    println!("Day 01");
    // let test_input = utils::read_test_input(1);
    let test_input = utils::read_input(1);
    println!("Test Input:\n{}", test_input);
    let parsed = parse_to_signed_numbers(&test_input);
    let mut password: u16 = 0;
    let mut dial: i16 = 50;
    for rotation in parsed {
        dial = (dial + rotation).rem_euclid(100);
        
        if dial == 0 {
            password += 1;
        }
    }
    
    println!("Password: {}", password);
}

fn parse_to_signed_numbers(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("R") {
                line[1..].parse::<i16>().unwrap()
            } else {
                -line[1..].parse::<i16>().unwrap()
            }
        })
        .collect()
}
