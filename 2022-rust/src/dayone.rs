use std::io::{BufRead, BufReader, Read};
use std::ops::Add;

#[derive(Debug)]
pub struct Elf {
    calories: Vec<Calories>
}

impl Elf {
    pub fn count_calories(&self) -> usize {
        return self.calories.iter().sum();
    }
}

pub type Calories = usize;

pub fn parse_elves(source: impl Read) -> Vec<Elf>
{
    let mut lines = BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok());

    let mut elves: Vec<Elf> = Vec::with_capacity(10);
    let mut cals = Vec::<Calories>::with_capacity(10);

    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line == "" {
            elves.push(Elf{ calories: cals});
            cals = Vec::with_capacity(10);
            continue;
        }

        cals.push(line.parse::<Calories>().unwrap())
    }

    return elves;
}

pub fn get_highest_calories(elves: &Vec<Elf>) -> usize {
    let mut highest = 0;
    for elf in elves {
        let count = elf.count_calories();
        if count > highest {
            highest = count;
        }
    }

    return highest;
}

pub fn get_top_three_calories(elves: &Vec<Elf>) -> usize {
    let mut calories: Vec<usize> = elves.iter().map(|e| e.count_calories()).collect();
    calories.sort_by(|a,b| b.cmp(a));
    let three = &calories[0..3];
    return three.iter().sum();
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn it_solves_part_one() {
        let file = File::open("data/input.txt").unwrap();

        let elves = parse_elves(file);
        let count = get_highest_calories(&elves);

        assert_eq!(65912, count);
    }

    #[test]
    fn it_solves_part_two() {
        let file = File::open("data/input.txt").unwrap();

        let mut elves = parse_elves(file);
        let count = get_top_three_calories(&mut elves);

        assert_eq!(195625, count);
    }
}
