use std::collections::HashSet;

fn load_input() -> String {
    return std::fs::read_to_string("input.txt").expect("File does not exists!");
}

fn find_start_seq(input: &String) -> u32 {
    for i in 0..(input.len() - 4) {
        let seq = input.chars().skip(i).take(4).collect::<HashSet<char>>();
        if seq.len() == 4 {
            return (i + 4) as u32;
        }
    }

    panic!("Sequence does not have unique 4 element subsequence!")
}

fn find_message_seq(input: &String) -> u32 {
    for i in 0..(input.len() - 14) {
        let seq = input.chars().skip(i).take(14).collect::<HashSet<char>>();
        if seq.len() == 14 {
            return (i + 14) as u32;
        }
    }

    panic!("Sequence does not have unique 4 element subsequence!")
}

fn main() {
    let input = load_input();
    let start_marker = find_start_seq(&input);
    let end_marker = find_message_seq(&input);
    println!("{}, {}", start_marker, end_marker);
}
