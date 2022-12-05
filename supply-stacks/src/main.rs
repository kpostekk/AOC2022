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
    let _x = SupplyStack::from_file();
    println!("Hello, world!");
}
