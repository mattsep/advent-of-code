#![allow(dead_code, unused_assignments)]

use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

type Dots = HashSet<Point>;
type Folds = Vec<Fold>;

fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Dots, Folds)> {
    let mut dots = Dots::new();
    let mut folds = Folds::new();

    let file = fs::File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line.unwrap();
        if line.starts_with("fold") {
            if let Some((axis, val)) = line[11..].split_once('=') {
                let val = val.parse::<i32>().unwrap();
                match axis {
                    "x" => folds.push(Fold::X(val)),
                    "y" => folds.push(Fold::Y(val)),
                    _ => panic!("Unknown axis: '{}'", axis),
                };
            }
        } else {
            if let Some((x, y)) = line.split_once(',') {
                let x = x.parse::<i32>().unwrap();
                let y = y.parse::<i32>().unwrap();
                dots.insert(Point { x, y });
            }
        }
    }

    Ok((dots, folds))
}

fn do_fold(dots: &mut Dots, fold: &Fold) {
    let reflected = dots
        .iter()
        .filter(|dot| match fold {
            Fold::X(x) => dot.x > *x,
            Fold::Y(y) => dot.y > *y,
        })
        .cloned()
        .collect::<Vec<Point>>();

    for dot in reflected {
        dots.remove(&dot);

        let mut dot = dot;
        match fold {
            Fold::X(x) => dot.x = 2 * x - dot.x,
            Fold::Y(y) => dot.y = 2 * y - dot.y,
        }

        dots.insert(dot);
    }
}

pub fn solve_it() {
    let (mut dots, folds) = parse("input/13.txt").unwrap();
    for fold in folds {
        do_fold(&mut dots, &fold);
    }

    let max_x = dots.iter().map(|dot| dot.x).max().unwrap();
    let max_y = dots.iter().map(|dot| dot.y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let dot = Point { x, y };
            if dots.contains(&dot) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
