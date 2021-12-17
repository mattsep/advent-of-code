#![allow(dead_code, unused_assignments)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

enum Size {
    Big,
    Small,
}

struct Node {
    index: usize,
    size: Size,
}

type Cave = Vec<Vec<Node>>;

fn parse<P: AsRef<Path>>(path: P) -> Cave {
    let mut cave = Vec::new();
    let mut nodes = HashMap::<&str, usize>::new();
    let mut index: usize = 0;

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if let Some((from, to)) = line.split_once('-') {
            let from_size = if from.chars().next().unwrap().is_lowercase() {
                Size::Small
            } else {
                Size::Big
            };

            let to_size = if to.chars().next().unwrap().is_lowercase() {
                Size::Small
            } else {
                Size::Big
            };

            if !nodes.contains_key(&from) {
                nodes.insert(&from, index);
                cave.push(vec![]);
                index += 1;
            }

            if !nodes.contains_key(&to) {
                nodes.insert(&to, index);
                cave.push(vec![]);
                index += 1;
            }

            let i = nodes[&from];
            let j = nodes[&to];

            let from = Node {
                index: i,
                size: from_size,
            };

            let to = Node {
                index: j,
                size: to_size,
            };

            cave[i].push(to);
            cave[j].push(from);
        }
    }

    cave
}

pub fn solve_it() {}
