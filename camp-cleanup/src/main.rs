use std::{fs, ops::RangeInclusive};

type PairsOfRanges = Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>;

fn load_sections() -> PairsOfRanges {
    let file = fs::read_to_string("input.txt").expect("input file not found!");

    let lines = file.lines();

    let ranges = lines
        .map(|line| {
            return line
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
        })
        .map(|vec_of_ranges| {
            let first_range = vec_of_ranges
                .get(0)
                .expect("No range on index 0!")
                .split("-")
                .collect::<Vec<&str>>();
            let second_range = vec_of_ranges
                .get(1)
                .expect("No value on index 1!")
                .split("-")
                .collect::<Vec<&str>>();

            let first = RangeInclusive::new(
                first_range.get(0).unwrap().parse::<u8>().unwrap(),
                first_range.get(1).unwrap().parse::<u8>().unwrap(),
            );

            let second = RangeInclusive::new(
                second_range.get(0).unwrap().parse::<u8>().unwrap(),
                second_range.get(1).unwrap().parse::<u8>().unwrap(),
            );

            return (first, second);
        })
        .collect::<Vec<(RangeInclusive<u8>, RangeInclusive<u8>)>>();
    return ranges;
}

fn find_fully_overlapped(pairs: PairsOfRanges) -> u16 {
    let mut count_of_full_overlaps = 0;

    for (first, second) in pairs {
        // check if first contains second
        if first.start() <= second.start() && first.end() >= second.end() {
            count_of_full_overlaps += 1;
        // check if second contains first
        } else if second.start() <= first.start() && second.end() >= first.end() {
            count_of_full_overlaps += 1;
        }
    }

    return count_of_full_overlaps;
}

fn find_overlaps(pairs: PairsOfRanges) -> u16 {
    let mut count_of_overlaps = 0;

    for (first, second) in pairs {
        let mut overlap_flag = false;

        // overlap check on first
        for i in first.clone() {
            if second.contains(&i) {
                overlap_flag = true;
                break;
            }
        }

        if overlap_flag {
            count_of_overlaps += 1;
            continue;
        }

        // overlap check on second
        for i in second.clone() {
            if first.contains(&i) {
                overlap_flag = true;
                break;
            }
        }

        if overlap_flag {
            count_of_overlaps += 1;
        }
    }

    return count_of_overlaps;
}

fn main() {
    let pairs = load_sections();
    let count = find_fully_overlapped(pairs.clone());
    println!("{}", count);

    let count_of_overlaps = find_overlaps(pairs.clone());
    println!("{}", count_of_overlaps);
}
