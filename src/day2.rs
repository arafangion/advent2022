use std::fs::read_to_string;

pub fn main() {
    let example = r#"A Y
B X
C Z
"#;

    println!("Day 2");
    println!("Examples:");
    part_a(example);
    part_b(example);

    println!("Actual:");
    let actual = read_to_string("./data/day2a.txt").unwrap();
    part_a(actual.as_str());
    part_b(actual.as_str());
}

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

enum RoundResult {
    Lose,
    Draw,
    Win
}

fn parse_hand(chr: u8) -> Hand {
    return match chr {
        b'A' => Hand::Rock,
        b'B' => Hand::Paper,
        b'C' => Hand::Scissors,

        b'X' => Hand::Rock,
        b'Y' => Hand::Paper,
        b'Z' => Hand::Scissors,
        _ => panic!("Unhandled")
    };
}

fn parse_round_result(chr: u8) -> RoundResult {
    return match chr {
        b'X' => RoundResult::Lose,
        b'Y' => RoundResult::Draw,
        b'Z' => RoundResult::Win,
        _ => panic!("Unhandled")
    };
}

fn score(round: &(Hand, Hand)) -> u32 {
    let outcome = match round {
        // Ties
        (Hand::Rock, Hand::Rock) |
        (Hand::Paper, Hand::Paper) |
        (Hand::Scissors, Hand::Scissors) => 3,

        // Winners
        (Hand::Rock, Hand::Paper) |
        (Hand::Paper, Hand::Scissors) |
        (Hand::Scissors, Hand::Rock) => 6,

        // Losers
        _ => 0
    };

    return outcome + match round.1 {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3
    };
}

fn guess_and_score(round: &(Hand, RoundResult)) -> u32 {
    return match round {
        (Hand::Rock, RoundResult::Win) => score(&(round.0, Hand::Paper)),
        (Hand::Paper, RoundResult::Win) => score(&(round.0, Hand::Scissors)),
        (Hand::Scissors, RoundResult::Win) => score(&(round.0, Hand::Rock)),

        (Hand::Rock, RoundResult::Lose) => score(&(round.0, Hand::Scissors)),
        (Hand::Paper, RoundResult::Lose) => score(&(round.0, Hand::Rock)),
        (Hand::Scissors, RoundResult::Lose) => score(&(round.0, Hand::Paper)),

        (_, RoundResult::Draw) => score(&(round.0, round.0)),
    };
}

fn parse(data: &str) -> Vec<(Hand, Hand)> {
    let mut rounds = Vec::new();
    for hand in data.trim().split("\n").map(|x| x.as_bytes()) {
        rounds.push((parse_hand(hand[0]), parse_hand(hand[hand.len()-1])))
    }

    return rounds;
}

fn parse_b(data: &str) -> Vec<(Hand, RoundResult)> {
    let mut rounds = Vec::new();
    for hand in data.trim().split("\n").map(|x| x.as_bytes()) {
        rounds.push((parse_hand(hand[0]), parse_round_result(hand[hand.len()-1])))
    }

    return rounds;
}

fn part_a(data: &str) {
    let rounds = parse(data);
    let score: u32 = rounds.iter().map(score).sum();
    println!("Score: {}", score);
}

fn part_b(data: &str) {
    let rounds = parse_b(data);
    let score: u32 = rounds.iter().map(guess_and_score).sum();
    println!("Score: {}", score);
}
