#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::FromIterator;
use std::path::Path;

const PEAK: i32 = 9;

type Point = [usize; 2];

struct Grid {
    data: Vec<i32>,
    size: [usize; 2],
}

impl Grid {
    pub fn with_size(m: usize, n: usize) -> Grid {
        let mut grid = Grid {
            data: Vec::with_capacity(m * n),
            size: [m, n],
        };
        for _ in 0..m * n {
            grid.data.push(0);
        }
        grid
    }

    pub fn get(&self, p: Point) -> Option<&i32> {
        let [i, j] = p;
        if i < self.size[0] && j < self.size[1] {
            Some(&self.data[self.ncols() * i + j])
        } else {
            None
        }
    }

    pub fn nrows(&self) -> usize {
        self.size[0]
    }
    pub fn ncols(&self) -> usize {
        self.size[1]
    }
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Grid> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    let mut data = vec![];
    let mut size = [0; 2];
    for line in lines {
        let line = line.unwrap();
        size[0] += 1;
        size[1] = line.len();
        for c in line.chars() {
            data.push(c.to_digit(10).unwrap() as i32);
        }
    }

    assert_eq!(data.len(), size[0] * size[1]);

    Ok(Grid {
        data: data,
        size: size,
    })
}

fn neighbors(p: Point) -> [Point; 4] {
    let [i, j] = p;
    [[i - 1, j], [i + 1, j], [i, j - 1], [i, j + 1]]
}

fn floodfill<Pred: Fn(Point) -> bool>(init: Point, pred: Pred) -> Vec<Point> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    let mut todo = Vec::new();
    seen.insert(init);
    todo.push(init);
    while !todo.is_empty() {
        let pt = todo.pop().unwrap();
        seen.insert(pt);

        if pred(pt) {
            result.push(pt);
            for neighbor in neighbors(pt) {
                if seen.insert(neighbor) {
                    todo.push(neighbor);
                }
            }
        }
    }

    result
}

pub fn solve_it() {
    let grid = parse("input/9.txt").unwrap();
    let mut basins = HashSet::new();
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            let height = grid.get([i, j]).unwrap();
            let mut is_smallest = true;
            for [ii, jj] in neighbors([i, j]) {
                if height >= grid.get([ii, jj]).unwrap_or(&PEAK) {
                    is_smallest = false;
                    break;
                }
            }

            if is_smallest {
                let mut basin = floodfill([i, j], |point| *grid.get(point).unwrap_or(&PEAK) < PEAK);
                basin.sort();
                basins.insert(basin);
            }
        }
    }

    let mut basins = Vec::from_iter(basins);
    basins.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut value = 1;
    for i in (basins.len() - 3)..(basins.len()) {
        value *= basins[i].len();
    }

    println!("{}", value);
}
