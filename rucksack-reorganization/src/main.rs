use std::{
    collections::{HashMap, HashSet},
    fs,
    marker::Copy,
    ops::Range,
    str::Chars,
};

#[derive(Clone)]
struct Rucksack {
    left_compartment: Vec<char>,
    right_compartment: Vec<char>,
}

impl Rucksack {
    fn from_string(input: String) -> Rucksack {
        if input.len() % 2 == 1 {
            panic!("Input for rucksack is odd.");
        }

        return Rucksack {
            left_compartment: input[0..(input.len() / 2)].chars().collect(),
            right_compartment: input[(input.len() / 2)..input.len()].chars().collect(),
        };
    }

    fn com_len(&self) -> usize {
        return self.right_compartment.len();
    }

    fn inside_both_compartments(&self, c: char) -> bool {
        return self.right_compartment.contains(&c) && self.left_compartment.contains(&c);
    }

    fn find_all_common(&self) -> HashSet<char> {
        let mut common_items = HashSet::<char>::new();

        for i in 0..self.com_len() {
            let l_item = self.left_compartment[i];
            let r_item = self.right_compartment[i];

            if self.inside_both_compartments(l_item) {
                common_items.insert(l_item);
            }
            if self.inside_both_compartments(r_item) {
                common_items.insert(r_item);
            }
        }

        return common_items;
    }

    fn item_types(&self) -> HashSet<char> {
        let mut types_in_rucksacks: HashSet<char> = HashSet::new();
        for i in 0..self.com_len() {
            types_in_rucksacks.insert(self.left_compartment[i]);
            types_in_rucksacks.insert(self.right_compartment[i]);
        }
        return types_in_rucksacks;
    }
}

fn main() {
    let mut priorities = HashMap::<char, u32>::new();

    // generate priorities for lowercase letters
    for priority in 1..=26 {
        priorities.insert(char::from_u32('a' as u32 - 1 + priority).unwrap(), priority);
    }

    for priority in 27..=52 {
        priorities.insert(
            char::from_u32('A' as u32 - 27 + priority).unwrap(),
            priority,
        );
    }

    let file_lines = fs::read_to_string("input.txt").expect("Rucksacks not found! 🧝🎒");
    let rucksacks: Vec<Rucksack> = file_lines
        .lines()
        .map(|line| Rucksack::from_string(line.to_string()))
        .collect();

    let mut items_in_both_compartments = HashMap::<char, u32>::new();

    for r in rucksacks.clone() {
        for common_item in r.find_all_common() {
            if !items_in_both_compartments.contains_key(&common_item) {
                items_in_both_compartments.insert(common_item, 0);
            }

            items_in_both_compartments.insert(
                common_item,
                items_in_both_compartments.get(&common_item).unwrap() + 1,
            );
        }
    }

    let whatever = items_in_both_compartments
        .iter()
        .map(|entry| priorities.get(entry.0).unwrap() * entry.1)
        .sum::<u32>();

    println!("{}", whatever);

    // task 2
    let mut groups = Vec::<char>::new();
    for r_chunk in rucksacks.clone().chunks(3) {
        for rl in r_chunk {
            
        }
    }
}
