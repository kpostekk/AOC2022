use std::collections::HashMap;

fn main() {
    let predefined_your_scores = HashMap::from([
        ("A X".to_string(), 4u32),
        ("B Y".to_string(), 5u32),
        ("C Z".to_string(), 6u32),
        ("A Y".to_string(), 8u32),
        ("A Z".to_string(), 3u32),
        ("B X".to_string(), 1u32),
        ("B Z".to_string(), 9u32),
        ("C X".to_string(), 7u32),
        ("C Y".to_string(), 2u32),
    ]);

    let mut score: u32 = 0;

    let binding = std::fs::read_to_string("input.txt").expect("File not found");
    let games = binding.lines();

    for game in games {
        if predefined_your_scores.contains_key(game) {
            score += predefined_your_scores.get(game).unwrap();
        }
    }
    
    println!("{}", score)
}
