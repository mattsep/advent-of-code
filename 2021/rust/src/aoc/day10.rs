#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
// use std::iter::zip;
// use std::collections::HashMap;

use phf::phf_map;

const OPEN_BRACES: [char; 4] = ['(', '[', '{', '<'];
const CLOSED_BRACES: [char; 4] = [')', ']', '}', '>'];

const SCORE_CORRUPT: phf::Map<char, u64> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};

const SCORE_INCOMPLETE: phf::Map<char, u64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
};

const AS_OPEN: phf::Map<char, char> = phf_map! {
    ')' => '(',
    ']' => '[',
    '}' => '{',
    '>' => '<',
};

const AS_CLOSED: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

enum BraceValidation {
    Incomplete(u64),
    Mismatch(u64),
    Complete,
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

fn score_line(line: &str) -> BraceValidation {
    let mut open = Vec::new();
    let mut mismatched = None;

    for c in line.chars() {
        if OPEN_BRACES.contains(&c) {
            open.push(c);
        } else {
            let expected = AS_CLOSED[&open.pop().unwrap()];
            if c != expected {
                // println!("Corrupt: found '{}' but expected '{}'", c, expected);
                mismatched = Some(c);
                break;
            }
        }
    }

    if let Some(brace) = mismatched {
        BraceValidation::Mismatch(SCORE_CORRUPT[&brace])
    } else {
        if open.is_empty() {
            BraceValidation::Complete
        } else {
            let mut score = 0;
            for brace in open.iter().rev() {
                score *= 5;
                score += SCORE_INCOMPLETE[&AS_CLOSED[brace]];
            }
            BraceValidation::Incomplete(score)
        }
    }
}

pub fn solve_it() {
    let lines = parse("input/10.txt").unwrap();

    let score_corrupt: u64 = lines
        .iter()
        .map(|line| score_line(line))
        .map(|result| match result {
            BraceValidation::Mismatch(score) => score,
            _ => 0,
        })
        .sum();
    
    let mut scores = lines
        .iter()
        .map(|line| score_line(line))
        .map(|result| match result {
            BraceValidation::Incomplete(score) => score,
            _ => 0,
        })
        .filter(|score| *score != 0)
        .collect::<Vec<_>>();
    
    scores.sort();
    let score_incomplete = scores[scores.len() / 2];

    println!("{}", score_corrupt);
    println!("{}", score_incomplete);
}
