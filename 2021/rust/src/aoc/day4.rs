#![allow(dead_code)]

use std::fs::{self};
use std::io::{self, BufRead};
use std::fmt::{self, Display};
use std::path::Path;

#[derive(Clone)]
struct Board([[(i32, bool); 5]; 5]);

impl Board {
    fn select(&mut self, value: i32) {
        for row in self.0.iter_mut() {
            for (v, s) in row.iter_mut() {
                if *v == value {
                    *s = true;
                }
            }
        }
    }
    
    // fn deselect(&mut self, i: usize, j: usize) {
    //     let v = self.0[i][j].0;
    //     self.0[i][j] = (v, false);
    // }
    
    // fn is_selected(&self, i: usize, j: usize) -> bool {
    //     self.0[i][j].1
    // }

    fn is_winner(&self) -> bool {
        let mut rows = [0; 5];
        let mut cols = [0; 5];
        for (i, row) in self.0.iter().enumerate() {
            for (j, (_, s)) in row.iter().enumerate() {
                rows[i] += *s as i32;
                cols[j] += *s as i32;
            }
        }
        rows.iter().any(|n| *n == 5) || cols.iter().any(|n| *n == 5)
    }

    fn sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        for row in self.0 {
            for (value, selected) in row {
                sum += if !selected { value } else { 0 };
            }
        }
        sum
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for row in self.0 {
            for (val, selected) in row {
                if selected {
                    write!(f, "({:2})", val)?;
                }
                else
                {
                    write!(f, " {:2} ", val)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<(Vec<i32>, Vec<Board>)> {
    let moves: Vec<i32>;
    let mut boards = Vec::<Board>::new();
    
    let file = fs::File::open(path)?;
    let mut lines = io::BufReader::new(file).lines();
    
    let line = lines.next().unwrap().unwrap();
    moves = line
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    
    let mut board = Board{ 0: [[(0, false); 5]; 5] };
    let mut row = 0;
    for line in lines {
        let line = line.unwrap();
        for (col, val) in line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).enumerate() {
            board.0[row][col] = (val, false);
        }
        if line.len() > 0 {
            row = (row + 1) % 5;
            if row == 0 {
                boards.push(board.clone());
            }
        }
    }

    Ok((moves, boards))
}

pub fn solve_it() {
    let (values, mut boards) = parse("input/4.txt").unwrap();

    let mut winners = vec![];
    
    for value in values {
        for (i, board) in boards.iter_mut().enumerate().filter(|(_, board)| !board.is_winner()) {
            board.select(value);
            if board.is_winner() {
                winners.push((i, value));
            }
        }
    }

    let (i, value) = winners.last().unwrap();
    println!("{}", boards[*i]);
    println!("score: {}", boards[*i].sum_unmarked() * value);
}