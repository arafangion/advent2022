use std::fs::read_to_string;

pub fn main() {
    let example = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    println!("Day 1");
    println!("Examples:");
    part_a(example);
    part_b(example);

    println!("Actual:");
    part_a(read_to_string("./data/day1a.txt").unwrap().as_str());
    part_b(read_to_string("./data/day1a.txt").unwrap().as_str());
}


pub fn part_a(data: &str) {
    let mut elfs: Vec<i32> = Vec::new();
    for foods in data.split("\n\n") {
        elfs.push(foods.split("\n").map(|x| x.parse::<i32>().unwrap_or(0)).sum());
    }

    let max_calories = *elfs.iter().max().unwrap();
    println!("Elf with max calories: {1} -> {0}", max_calories, elfs.iter().position(|&e| e == max_calories).unwrap()+1);

}

pub fn part_b(data: &str) {
    let mut elfs: Vec<i32> = Vec::new();
    for foods in data.split("\n\n") {
        elfs.push(foods.split("\n").map(|x| x.parse::<i32>().unwrap_or(0)).sum());
    }

    let mut sorted = elfs;
    sorted.sort();
    println!("Total calories carried by top 3 elves are: {}", sorted[sorted.len()-3..].iter().sum::<i32>());
}
