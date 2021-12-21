#![allow(dead_code, unused_assignments)]

use std::cmp::{Ordering};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Grid = Vec<Vec<usize>>;
type Point = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    index: Point,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn nrows(grid: &Grid) -> usize {
    grid.len()
}

fn ncols(grid: &Grid) -> usize {
    if let Some(row) = grid.get(0) {
        row.len()
    } else {
        0
    }
}

fn wrap(n: usize) -> usize {
    1 + (n - 1) % 9
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Grid> {
    let mut grid = Grid::new();
    let file = File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let row = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        grid.push(row);
    }
    // Ok(grid)

    // PART 2
    let mut big_grid = vec![vec![0; 5 * ncols(&grid)]; 5 * nrows(&grid)];
    for m in 0..5 {
        for n in 0..5 {
            for i in 0..nrows(&grid) {
                for j in 0..ncols(&grid) {
                    let ii = m * nrows(&grid) + i;
                    let jj = n * ncols(&grid) + j;
                    big_grid[ii][jj] = wrap(grid[i][j] + m + n);
                }
            }
        }
    }

    Ok(big_grid)
}

fn neighbors(grid: &Grid, ij: Point) -> [Option<Point>; 4] {
    let (i, j) = ij;
    let (m, n) = (nrows(grid), ncols(grid));
    [
        if i > 0 { Some((i - 1, j)) } else { None },
        if j > 0 { Some((i, j - 1)) } else { None },
        if i < m - 1 { Some((i + 1, j)) } else { None },
        if j < n - 1 { Some((i, j + 1)) } else { None },
    ]
}

fn find_shortest_path(grid: &Grid) -> Option<usize> {
    // Dijkstra's Algorithm
    let mut best = vec![vec![usize::MAX; ncols(grid)]; nrows(grid)];
    best[0][0] = grid[0][0];

    let mut heap = BinaryHeap::new();
    heap.push(State {
        index: (0, 0),
        cost: 0,
    });

    let goal = (nrows(grid) - 1, ncols(grid) - 1);
    while let Some(State { index, cost }) = heap.pop() {
        let (i, j) = index;

        if index == goal {
            return Some(cost);
        }

        if cost > best[i][j] {
            continue;
        }

        for next in neighbors(grid, index) {
            if let Some((k, l)) = next {
                let next = State {
                    index: (k, l),
                    cost: cost + grid[k][l],
                };

                if next.cost < best[k][l] {
                    heap.push(next);
                    best[k][l] = next.cost;
                }
            }
        }
    }

    None
}

pub fn solve_it() {
    let grid = parse("input/15.txt").unwrap();
    let cost = find_shortest_path(&grid);
    println!("{:?}", cost);
}
