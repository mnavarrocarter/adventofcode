use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};
use std::ops::{Add, Sub};

// Parses Rucksacks from input
pub fn parse_rucksacks(source: impl Read) -> Vec<String> {
    return BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok())
        .collect();
}

// Sums the repeated items priority
pub fn sum_priority_repeated_items(rucksacks: &Vec<String>) -> usize {
    let mut sum: usize = 0;
    for rucksack in rucksacks {
        let c = find_repeated_item(&rucksack).unwrap();
        let priority = get_priority(c);
        sum = sum.add(priority)
    }

    return sum;
}

// Sums the group badges priority
pub fn sum_priority_of_group_badges(rucksacks: &Vec<String>) -> usize {
    let mut sum: usize = 0;
    let mut prev: String = "".to_string();

    for (i, rucksack) in rucksacks.into_iter().enumerate() {
        if prev == "" {
            prev = rucksack.clone();
            continue;
        }

        prev = str_intersect(&prev, rucksack);

        // Every three iterations
        if (i + 1) % 3 == 0 {
            let c = prev.to_string().pop().unwrap();
            sum = sum.add(get_priority(c));
            prev = "".to_string();
        }
    }

    return sum;
}

// Finds a repeated item in the rucksack
fn find_repeated_item(rucksack: &String) -> Option<char> {
    let len = rucksack.len();

    let (a, b) = rucksack.split_at(len / 2);
    let mut items = str_intersect(&a.to_string(), &b.to_string());
    return items.pop()
}

// Returns a new string with the characters present both in a and b
fn str_intersect(a: &String, b : &String) -> String {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    } else {
        (a, b)
    };

    let set: HashSet<char> = shorter.chars().collect();
    let mut intersection: HashSet<char> = HashSet::with_capacity(shorter.len() / 2 as usize);

    for c in longer.chars() {
        if set.contains(&c) {
            intersection.insert(c);
        }
    }

    return intersection.into_iter().collect();
}

// Gets the priority of an item
// It does so by doing some math on the unicode values
fn get_priority(c: char) -> usize {
    let unicode = c as u32;

    // A (65) to Z (90)
    if unicode >= 65 && unicode <= 90 {
        return unicode.sub(38) as usize;
    }

    // a (97) to z (122)
    if unicode >= 97 && unicode <= 122 {
        return unicode.sub(96) as usize;
    }

    return 0
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn it_passes_part_one() {
        let file = File::open("data/day03.test.txt").unwrap();
        let rucksacks = parse_rucksacks(file);
        let sum = sum_priority_repeated_items(&rucksacks);

        assert_eq!(157, sum);
    }

    #[test]
    fn it_solves_part_one() {
        let file = File::open("data/day03.txt").unwrap();
        let rucksacks = parse_rucksacks(file);
        let sum = sum_priority_repeated_items(&rucksacks);

        assert_eq!(8072, sum);
    }

    #[test]
    fn it_passes_part_two() {
        let file = File::open("data/day03.test.txt").unwrap();
        let rucksacks = parse_rucksacks(file);
        let sum = sum_priority_of_group_badges(&rucksacks);

        assert_eq!(70, sum);
    }

    #[test]
    fn it_solves_part_two() {
        let file = File::open("data/day03.txt").unwrap();
        let rucksacks = parse_rucksacks(file);
        let sum = sum_priority_of_group_badges(&rucksacks);

        assert_eq!(2567, sum);
    }
}