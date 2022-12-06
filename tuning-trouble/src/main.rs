use std::collections::HashSet;

fn load_input() -> String {
    return std::fs::read_to_string("input.txt").expect("File does not exists!");
}

fn find_unique_seq(input: &String, seq_size: usize) -> Option<usize> {
    for i in 0..(input.len() - seq_size) {
        let seq = input.chars().skip(i).take(seq_size).collect::<HashSet<char>>();
        if seq.len() == seq_size {
            return Some(i + seq_size);
        }
    }
    return None;
}

fn main() {
    let input = load_input();
    let start_marker = find_unique_seq(&input, 4).expect("Unable to find subsequence!");
    let end_marker = find_unique_seq(&input, 14).expect("Unable to find subsequence!");
    println!("{}, {}", start_marker, end_marker);
}
