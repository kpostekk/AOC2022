use regex::Regex;

struct Stacks {
    stacks: Vec<SupplyStack>,
}

impl Stacks {
    fn create(stacks: Vec<SupplyStack>) -> Stacks {
        return Stacks { stacks };
    }

    fn perform_command(&self, command: String) {
        let count_regex = Regex::new(r"move (\d+)").unwrap();
        let src_dst_regex = Regex::new(r"from (\d+) to (\d+)").unwrap();
        
        let count = count_regex.find(&command).expect("Unable to find move declaration").as_str();
        let src = src_dst_regex.find(&command).expect("Unable to find from/todeclaration").as_str();
        
        print!("{}, {}", count, src)
    }
}

struct SupplyStack {
    stack: Vec<char>,
}

impl SupplyStack {
    fn from_file() -> Vec<SupplyStack> {
        let input = std::fs::read_to_string("stacks.txt").expect("Unable to find file!");

        let mut supplies = Vec::<SupplyStack>::new();

        for line in input.lines() {
            let mut stack = Vec::<char>::new();

            for c in line.chars() {
                stack.push(c);
            }

            supplies.push(SupplyStack { stack });
        }

        return supplies;
    }
}

fn main() {
    let x = Stacks::create(SupplyStack::from_file());
    x.perform_command("move 2 from 8 to 1".to_string());
    println!("Hello, world!");
}
