use std::io::{BufRead, BufReader, Read};
use std::ops::Add;

#[derive(Debug)]
struct Elf {
    calories: usize
}

fn parse_elves(source: impl Read) -> Vec<Elf>
{
    let mut lines = BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok());

    let mut elves: Vec<Elf> = Vec::with_capacity(10);
    let mut cals: usize = 0;

    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line == "" {
            elves.push(Elf{ calories: cals});
            cals = 0;
            continue;
        }

        cals = cals.add(line.parse::<usize>().unwrap());
    }

    return elves;
}

pub fn get_highest_calories(input: impl Read) -> usize {
    let elves = parse_elves(input);

    let mut highest = 0;
    for elf in elves {
        let count = elf.calories;
        if count > highest {
            highest = count;
        }
    }

    return highest;
}

pub fn get_top_three_calories(input: impl Read) -> usize {
    let elves = parse_elves(input);
    let mut calories: Vec<usize> = elves.iter().map(|e| e.calories).collect();
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
        let file = File::open("data/day01.txt").unwrap();
        let count = get_highest_calories(file);

        assert_eq!(65912, count);
    }

    #[test]
    fn it_solves_part_two() {
        let file = File::open("data/day01.txt").unwrap();
        let count = get_top_three_calories(file);

        assert_eq!(195625, count);
    }
}
