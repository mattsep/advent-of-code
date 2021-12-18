#![allow(dead_code)]

use std::fs;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

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
    let (mut poly, rules) = parse("input/14.txt").unwrap();

    for _ in 0..10 {
        poly = poly
            .iter()
            .zip(poly[1..].iter())
            .map(|(a, b)| {
                if let Some(c) = rules.get(&(*a, *b)) {
                    [*a, *c]
                } else {
                    [*a, *b]
                }
            })
            .flatten()
            .chain([*poly.last().unwrap()])
            .collect();
    }

    let mut counts = HashMap::new();
    for item in poly {
        *counts.entry(item).or_insert(0) += 1;
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("{}", max - min);
}