fn main() {
    println!("Day 07");

    // Swap to `read_input(7)` once you have the real puzzle input.
    let input = utils::read_test_input(7);
    let lines = input
        .split('\n')
        .enumerate()
        .map(|(index, line)| {
            let test = line.char_indices().collect();
            return (index, "test".to_string());
        })
        .collect::<Vec<(usize, String)>>();
    println!("Input:\n{}", input);
    println!("Input:\n{:?}", lines);
}
