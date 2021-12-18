#![allow(dead_code, unused_assignments)]

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Size {
    Big,
    Small,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    index: usize,
    size: Size,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.size == Size::Small { '*' } else { ' ' },
            self.index
        )
    }
}

type Cave = HashMap<Node, Vec<Node>>;

fn show<T: Display>(vec: &Vec<T>) {
    print!("[");
    for item in vec {
        print!("{}, ", *item);
    }
    println!("]");
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Cave, Node, Node)> {
    let mut cave = HashMap::new();
    let mut nodes = HashMap::<String, Node>::new();
    let mut index: usize = 0;

    fn get_size(name: &str) -> Size {
        if name.chars().next().unwrap().is_lowercase() {
            Size::Small
        } else {
            Size::Big
        }
    }

    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        if let Some((from, to)) = line.split_once('-') {
            let from = if !nodes.contains_key(from) {
                let node = Node {
                    index: index,
                    size: get_size(from),
                };
                nodes.insert(from.into(), node);
                index += 1;

                node
            } else {
                nodes[from]
            };

            let to = if !nodes.contains_key(to) {
                let node = Node {
                    index: index,
                    size: get_size(to),
                };
                nodes.insert(to.into(), node);
                index += 1;

                node
            } else {
                nodes[to]
            };

            cave.entry(from).or_insert(vec![]).push(to);
            cave.entry(to).or_insert(vec![]).push(from);
        }
    }

    Ok((cave, nodes["start"], nodes["end"]))
}

fn count_paths(cave: &Cave, source: Node, target: Node) -> usize {
    let mut result = 0;
    let mut todo = vec![source];
    let mut path = vec![];
    let mut cnts = vec![1];
    let mut num_visits = vec![0; cave.len()];

    while let Some(node) = todo.pop() {
        path.push(node);

        num_visits[node.index] += 1;
        if let Some(cnt) = cnts.last_mut() {
            *cnt -= 1;
        }

        if node == target {
            result += 1;
            num_visits[node.index] -= 1;
            path.pop();
        } else {
            let mut cnt = 0;
            for next in cave.get(&node).unwrap() {
                if *next == source {
                    continue;
                }
                if next.size == Size::Small && num_visits[next.index] >= 1 {
                    let max = cave
                        .keys()
                        .filter(|node| node.size == Size::Small)
                        .map(|node| num_visits[node.index])
                        .max()
                        .unwrap();
                    if max >= 2 {
                        continue;
                    }
                }

                cnt += 1;
                todo.push(*next);
            }
            cnts.push(cnt);
        }

        while let Some(cnt) = cnts.last() {
            if *cnt == 0 {
                cnts.pop();
                if let Some(node) = path.pop() {
                    num_visits[node.index] -= 1;
                }
            } else {
                break;
            }
        }
    }

    result
}

pub fn solve_it() {
    let (cave, source, target) = parse("input/12.txt").unwrap();
    println!("{}", count_paths(&cave, source, target));
}
