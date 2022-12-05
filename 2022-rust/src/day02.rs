use std::io::{BufRead, BufReader, Read};
use std::ops::Add;

pub fn calculate_total_score_with_guess(input: impl Read) -> usize {
    let rounds = parse_rounds(input);

    return calculate_total_score(rounds)
}

pub fn calculate_total_score_correctly(input: impl Read) -> usize {
    let hand_results = parse_hands_with_result(input);
    let mut rounds: Vec<(Hand, Hand)> = Vec::with_capacity(hand_results.capacity());

    for hand_result in hand_results {
        let my_hand = hand_result.1.against(&hand_result.0);
        let round = (hand_result.0, my_hand);
        rounds.push(round)
    }

    return calculate_total_score(rounds);
}

fn calculate_total_score(rounds: Vec<(Hand, Hand)>) -> usize {
    let mut score: usize = 0;

    for round in rounds {
        score = score.add(match round.1 {
            Hand::Rock => { 1 }
            Hand::Paper => { 2 }
            Hand::Scissors => { 3 }
        });

        score = score.add(match round.1.against(&round.0) {
            Game::Win => { 6 }
            Game::Draw => { 3 }
            Game::Loss => { 0 }
        })
    }

    return score
}

#[derive(Debug)]
enum Game {
    Win,
    Draw,
    Loss
}

impl Game {
    fn against(&self, op: &Hand) -> Hand {
        match op {
            Hand::Rock => {
                match self {
                    Game::Draw => { Hand::Rock }
                    Game::Loss => { Hand::Scissors }
                    Game::Win => { Hand::Paper }
                }
            }
            Hand::Paper => {
                match self {
                    Game::Draw => { Hand::Paper }
                    Game::Loss => { Hand::Rock }
                    Game::Win => { Hand::Scissors }
                }
            }
            Hand::Scissors => {
                match self {
                    Game::Draw => { Hand::Scissors }
                    Game::Loss => { Hand::Paper }
                    Game::Win => { Hand::Rock }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    // Returns the result of the game for self against op
    fn against(&self, op: &Hand) -> Game {
        match self {
            Hand::Rock => {
                match op {
                    Hand::Rock => { Game::Draw }
                    Hand::Paper => { Game::Loss }
                    Hand::Scissors => { Game::Win }
                }
            }
            Hand::Paper => {
                match op {
                    Hand::Rock => { Game::Win }
                    Hand::Paper => { Game::Draw }
                    Hand::Scissors => { Game::Loss }
                }
            }
            Hand::Scissors => {
                match op {
                    Hand::Rock => { Game::Loss }
                    Hand::Paper => { Game::Win }
                    Hand::Scissors => { Game::Draw }
                }
            }
        }
    }
}

fn parse_rounds(source: impl Read) -> Vec<(Hand, Hand)>
{
    let mut lines = BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok());

    let mut rounds: Vec<(Hand, Hand)> = Vec::with_capacity(10);

    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let opponent = match parts[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("invalid hand")
        };

        let mine = match parts[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => panic!("invalid hand")
        };

        rounds.push(( opponent, mine ))
    }

    return rounds;
}

fn parse_hands_with_result(source: impl Read) -> Vec<(Hand, Game)>
{
    let mut lines = BufReader::new(source)
        .lines()
        .filter_map(|l| l.ok());

    let mut rounds: Vec<(Hand, Game)> = Vec::with_capacity(10);

    loop {
        let line = match lines.next() {
            Some(s) => s,
            None => break,
        };

        if line == "" {
            continue;
        }

        let parts: Vec<&str> = line.trim().split(' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let opponent = match parts[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("invalid hand")
        };

        let result = match parts[1] {
            "X" => Game::Loss,
            "Y" => Game::Draw,
            "Z" => Game::Win,
            _ => panic!("invalid game")
        };

        rounds.push(( opponent, result ))
    }

    return rounds;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn it_passes_part_one() {
        let file = File::open("data/day02.test.txt").unwrap();

        let count = calculate_total_score_with_guess(file);

        assert_eq!(15, count);
    }

    #[test]
    fn it_solves_part_one() {
        let file = File::open("data/day02.txt").unwrap();

        let count = calculate_total_score_with_guess(file);

        assert_eq!(12535, count);
    }

    #[test]
    fn it_passes_part_two() {
        let file = File::open("data/day02.test.txt").unwrap();

        let count = calculate_total_score_correctly(file);

        assert_eq!(12, count);
    }

    #[test]
    fn it_solves_part_two() {
        let file = File::open("data/day02.txt").unwrap();

        let count = calculate_total_score_correctly(file);

        assert_eq!(15457, count);
    }
}
