#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

type Template = Vec<char>;
type Rules = HashMap<(char, char), char>;

fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Template, Rules)> {
    let mut template = Template::new();
    let mut rules = Rules::new();

    let file = fs::File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    for (i, line) in lines.enumerate() {
        let line = line.unwrap();
        if i == 0 {
            template = line.chars().collect();
        } else if let Some((ab, c)) = line.split_once(" -> ") {
            let mut ab = ab.chars();
            let a = ab.next().unwrap();
            let b = ab.next().unwrap();
            let c = c.chars().next().unwrap();
            rules.insert((a, b), c);
        }
    }

    Ok((template, rules))
}

pub fn solve_it() {
    let (poly, rules) = parse("input/14.txt").unwrap();
    let mut pairs = HashMap::new();

    // first and last characters are UNDERCOUNTED
    let first = *poly.first().unwrap();
    let last = *poly.last().unwrap();

    for (a, b) in poly.iter().zip(poly[1..].iter()) {
        *pairs.entry((*a, *b)).or_insert(0u64) += 1;
    }

    println!("{:?}", pairs);

    for _ in 0..40 {
        pairs = {
            let mut new_pairs = pairs.clone();
            for (a, b) in pairs.keys() {
                let ab = (*a, *b);
                if let Some(c) = rules.get(&ab) {
                    let ac = (*a, *c);
                    let cb = (*c, *b);
                    *new_pairs.entry(ac).or_insert(0) += pairs[&ab];
                    *new_pairs.entry(cb).or_insert(0) += pairs[&ab];
                    *new_pairs.entry(ab).or_insert(pairs[&ab]) -= pairs[&ab];
                }
            }

            new_pairs
        }
    }

    let mut counts = HashMap::new();
    counts.entry(first).or_insert(1u64);
    counts.entry(last).or_insert(1);

    for pair in pairs.keys() {
        let (a, b) = *pair;
        *counts.entry(a).or_insert(0) += pairs[pair];
        *counts.entry(b).or_insert(0) += pairs[pair];
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("{}", (max - min) / 2);
}
