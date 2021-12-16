#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct Observation {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<Observation>> {
    let mut result = vec![];
    
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut input = line
            .split('|')
            .map(|part| part
                .split_whitespace()
                .map(|part| String::from(part))
                .collect::<Vec<_>>());
        
        result.push(Observation{
            patterns: input.next().unwrap(),
            output: input.next().unwrap()
        });
    }
    
    Ok(result)
}

fn map_to_value(pattern: &str) -> Option<i32> {
    match pattern {
        "ABCEFG"  => Some(0),
        "CF"      => Some(1),
        "ACDEG"   => Some(2),
        "ACDFG"   => Some(3),
        "BCDF"    => Some(4),
        "ABDFG"   => Some(5),
        "ABDEFG"  => Some(6),
        "ACF"     => Some(7),
        "ABCDEFG" => Some(8),
        "ABCDFG"  => Some(9),
        _         => None,
    }
}

fn map_to_segments(patterns: &Vec<String>) -> HashMap<char, char> {
    let mut result = HashMap::new();
    let mut counts = HashMap::new();
    let mut lengths = HashMap::new();

    for pattern in patterns {
        for c in pattern.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        lengths.entry(pattern.len()).or_insert(vec![pattern]).push(pattern);
    }

    for (key, value) in counts.iter() {
        match *value {
            4 => { result.insert(*key, 'E'); },
            6 => { result.insert(*key, 'B'); },
            9 => { result.insert(*key, 'F'); },
            _ => { },
        }
    }
    
    for pattern in &lengths[&2] {
        for c in pattern.chars() {
            if !result.contains_key(&c) {
                result.insert(c, 'C');
                break;
            }
        }
    }

    for pattern in &lengths[&4] {
        for c in pattern.chars() {
            if !result.contains_key(&c) {
                result.insert(c, 'D');
                break;
            }
        }
    }

    for (key, value) in counts.iter() {
        if !result.contains_key(key) {
            result.insert(*key, match value {
                7 => Some('G'),
                8 => Some('A'),
                _ => None,
            }.unwrap());
        }
    }

    result
}

pub fn solve_it() {
    let observations = parse("input/8.txt").unwrap();
    
    let mut numbers = vec![];
    for observation in observations {
        let patterns = &observation.patterns;
        let segments = map_to_segments(patterns);

        let number: i32 = observation.output
            .iter()
            .map(|pattern| {
                let mut pattern = pattern
                    .chars()
                    .map(|c| segments[&c])
                    .collect::<Vec<_>>();
                pattern.sort();
                pattern.iter().collect::<String>() })
            .map(|pattern| map_to_value(&pattern))
            .enumerate()
            .map(|(i, n)| n.unwrap() * 10i32.pow(3 - i as u32))
            .sum();

        numbers.push(number);
    }
    
    println!("{}", numbers.iter().sum::<i32>());
}