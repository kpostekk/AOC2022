enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn from_string(pat: String) -> Option<RockPaperScissors> {
        if vec!["A", "X"].contains(&pat.as_str()) {
            return Some(RockPaperScissors::Rock);
        } else if vec!["B", "Y"].contains(&pat.as_str()) {
            return Some(RockPaperScissors::Paper);
        } else if vec!["C", "Z"].contains(&pat.as_str()) {
            return Some(RockPaperScissors::Scissors);
        } else {
            return None;
        }
    }
}

struct Round {
    yours: RockPaperScissors,
    theirs: RockPaperScissors,
}

impl Round {
    fn from_file(filename: String) {
        let _lines = std::fs::read_to_string(filename)
            .expect("Unable to read elve's 🪨🗞️✂️ tactic!")
            .lines()
            .map(|l| {
                let pair = l.split(" ").collect::<Vec<&str>>();
                return Round {
                    theirs: RockPaperScissors::from_string(pair.get(0).expect("Invalid line format!").to_string()).expect("Invalid line content!"),
                    yours: RockPaperScissors::from_string(pair.get(1).expect("Invalid line format!").to_string()).expect("Invalid line content!"),
                };
            }).collect::<Vec<Round>>();
        print!("")
    }
}

fn main() {
    Round::from_file("input.txt".to_string());
    println!("Hello, world!");
}
