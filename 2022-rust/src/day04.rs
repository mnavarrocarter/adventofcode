use std::io::{BufRead, BufReader, Read};
use std::ops::{Add};

#[derive(Debug)]
pub struct Assignment {
    from: usize,
    to: usize
}

impl Assignment {
    fn fully_contains(&self, b: &Assignment) -> bool {
        self.from <= b.from && self.to >= b.to
    }

    fn overlaps(&self, b: &Assignment) -> bool {
        (b.from >= self.from && b.from <= self.to)
            ||
        (b.to >= self.from && b.to <= self.to)
    }
}

// Parses Rucksacks from input
pub fn parse_assignment_pairs(source: impl Read) -> Vec<(Assignment, Assignment)> {
    let lines = BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok())
        .into_iter();

    let mut assignment_pairs: Vec<(Assignment, Assignment)> = Vec::with_capacity(10);

    for line in lines {
        let split: Vec<&str> = line.split(',').collect();

        if split.len() != 2 {
            continue; // Bad line
        }

        let numbers0: Vec<usize> = split[0].split('-').filter_map(|n| n.parse::<usize>().ok()).collect();
        if numbers0.len() != 2 {
            continue; // Bad line
        }

        let numbers1: Vec<usize> = split[1].split('-').filter_map(|n| n.parse::<usize>().ok()).collect();
        if numbers1.len() != 2 {
            continue; // Bad line
        }

        assignment_pairs.push((
            Assignment{from: numbers0[0], to: numbers0[1]},
            Assignment{from: numbers1[0], to: numbers1[1]}
        ));
    }

    assignment_pairs
}

pub fn count_fully_contained_pairs(list: &Vec<(Assignment, Assignment)>) -> usize {
    let mut count: usize = 0;
    for pair in list {
        if pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0) {
            count = count.add(1)
        }
    }

    return count;
}

pub fn count_overlapping_pairs(list: &Vec<(Assignment, Assignment)>) -> usize {
    let mut count: usize = 0;
    for pair in list {
        if pair.0.fully_contains(&pair.1) || pair.1.fully_contains(&pair.0) {
            count = count.add(1);
            continue;
        }

        if pair.0.overlaps(&pair.1) {
            count = count.add(1);
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn it_passes_part_one() {
        let file = File::open("data/day04.test.txt").unwrap();
        let assignment_pairs = parse_assignment_pairs(file);
        let count = count_fully_contained_pairs(&assignment_pairs);

        assert_eq!(2, count);
    }

    #[test]
    fn it_solves_part_one() {
        let file = File::open("data/day04.txt").unwrap();
        let assignment_pairs = parse_assignment_pairs(file);
        let count = count_fully_contained_pairs(&assignment_pairs);

        assert_eq!(599, count);
    }

    #[test]
    fn it_passes_part_two() {
        let file = File::open("data/day04.test.txt").unwrap();
        let assignment_pairs = parse_assignment_pairs(file);
        let count = count_overlapping_pairs(&assignment_pairs);

        assert_eq!(928, count);
    }

    #[test]
    fn it_solves_part_two() {
        let file = File::open("data/day04.txt").unwrap();
        let assignment_pairs = parse_assignment_pairs(file);
        let count = count_overlapping_pairs(&assignment_pairs);

        assert_eq!(928, count);
    }
}