use std::fs;

fn read_data_from_file() -> Vec<Vec<u32>> {
    let data = fs::read_to_string("input.txt").expect("Can't open file with elves' backpacks! 🧝");
    let lines = data.lines();
    let mut calories: Vec<Vec<u32>> = Vec::new();

    let mut temp_vector: Vec<u32> = Vec::new();
    for line in lines {
        if line.len() == 0 { // detect empty lines
            calories.push(temp_vector.to_vec());
            temp_vector.clear();
        } else {
            temp_vector.push(line.parse::<u32>().unwrap());
        }
    }

    // do the cleanup (if no new line on the end of input.txt)
    if temp_vector.len() > 0 {
        calories.push(temp_vector.to_vec());
    }

    return calories;
}

fn main() {
    let x = read_data_from_file();
    let y = x
        .iter()
        .reduce(|accum, next| {
            if accum.iter().sum::<u32>() > next.iter().sum::<u32>() {
                accum
            } else {
                next
            }
        })
        .unwrap();
    println!("{}", y.iter().sum::<u32>());
    let mut z = x
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    z.sort_by(|x, y| y.cmp(x));
    println!("{}", z.iter().take(3).sum::<u32>().to_string());
}
