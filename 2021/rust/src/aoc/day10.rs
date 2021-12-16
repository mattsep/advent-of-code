#![allow(dead_code)]

use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
// use std::iter::zip;
use std::collections::HashMap;

use phf::phf_map;

const OPEN_BRACES: [char; 4] = ['(', '[', '{', '<'];
const CLOSED_BRACES: [char; 4] = [')', ']', '}', '>'];

const SCORE_TABLE: phf::Map<char, i32> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
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

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

fn find_corrupt_chunk(line: &str) {
    let mut open = Vec::new();
    let mut mismatched = Vec::new();

    for c in line.chars() {
        if OPEN_BRACES.contains(&c) {
            open.push(c);
        } else {
            let expected = AS_CLOSED[&open.pop().unwrap()];
            if c != expected {
                println!("Corrupt: found '{}' but expected '{}'", c, expected);
                mismatched.push(c);
            }
        }
    }

    if !open.is_empty() && mismatched.is_empty() {
        println!("Line is incomplete!");
    } else if !mismatched.is_empty() {
        let score: i32 = mismatched
            .iter()
            .map(|c| SCORE_TABLE[&c])
            .sum();
    
        println!("{}", score);
    }
}

pub fn solve_it() {
    let lines = parse("input/test.txt").unwrap();
    for line in lines {
        find_corrupt_chunk(&line);
    }
}