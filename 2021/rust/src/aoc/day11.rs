#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const MAX_ENERGY: u32 = 9;
const WIDTH: usize = 10;

type Grid = [[u32; WIDTH]; WIDTH];
type Point = (usize, usize);

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Grid> {
    let mut result: Grid = [[0; WIDTH]; WIDTH];
    let mut row: usize = 0;
    let mut col: usize = 0;

    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        col = 0;
        for c in line.unwrap().chars() {
            result[row][col] = c.to_digit(10).unwrap();
            col += 1;
        }
        row += 1;
    }

    Ok(result)
}

fn neighbors(ij: Point) -> [Option<Point>; 8] {
    let mut result = [None; 8];
    let mut select = 0;
    let (i, j) = ij;

    for ii in (i - 1) as isize..=(i + 1) as isize {
        for jj in (j - 1) as isize..=(j + 1) as isize {
            let ii = ii as usize;
            let jj = jj as usize;
            if (i, j) != (ii, jj) {
                if ii < WIDTH && jj < WIDTH {
                    result[select] = Some((ii, jj));
                }
                select += 1;
            }
        }
    }

    result
}

fn update(grid: &mut Grid) -> bool {
    let mut seen = [[false; WIDTH]; WIDTH];
    let mut todo = Vec::with_capacity(WIDTH * WIDTH);

    for i in 0..WIDTH {
        for j in 0..WIDTH {
            grid[i][j] += 1;
            if grid[i][j] > MAX_ENERGY {
                todo.push((i, j));
                seen[i][j] = true;
            }
        }
    }

    while let Some((i, j)) = todo.pop() {
        for neighbor in neighbors((i, j)) {
            if let Some((i, j)) = neighbor {
                grid[i][j] += 1;
                if grid[i][j] > MAX_ENERGY && !seen[i][j] {
                    seen[i][j] = true;
                    todo.push((i, j));
                }
            }
        }
    }

    let mut num_flashes: usize = 0;
    for i in 0..WIDTH {
        for j in 0..WIDTH {
            if seen[i][j] {
                grid[i][j] = 0;
                num_flashes += 1;
            }
        }
    }
    
    num_flashes == WIDTH * WIDTH
}

pub fn solve_it() {
    let mut grid = parse("input/11.txt").unwrap();

    // PART 1
    // let mut cnt = 0;
    // for _ in 0..100 {
    //     cnt += update(&mut grid);
    // }
    //
    // println!("{}", cnt);

    // PART 2
    let mut step: usize = 0;
    loop {
        step += 1;
        if update(&mut grid) {
            break;
        }
    }

    println!("{}", step);
}
