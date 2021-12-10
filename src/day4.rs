use crate::base::read_lines;
use crate::day4::Entry::{Matched, Value};
use crate::DayExecutable;

pub struct Day4Executable {}

impl DayExecutable for Day4Executable {
    fn execute(&self) {
        let lines_result = read_lines("./data/day4.txt");
        match lines_result {
            Err(e) => println!("Error: {}", e),
            Ok(lines) => self.process(lines),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Entry {
    Value(u32),
    Matched,
}

#[derive(Debug, Clone)]
struct Board {
    has_won: bool,
    rows: Vec<Vec<Entry>>,
}

impl Day4Executable {
    fn process(&self, lines: Vec<String>) {
        let numbers: Vec<u32> = lines
            .get(0)
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let board_lines = &lines[2..];
        let mut boards = init_boards(board_lines);
        let mut last_winning_board: Option<Board> = None;
        let mut last_called_number = 0;
        for number in numbers {
            for board in boards.iter_mut() {
                if !board.has_won {
                    mark_number(board, number);
                    if check_winning_board(board) {
                        board.has_won = true;
                        last_winning_board = Some(board.clone());
                    }
                }
            }
            let exists_non_winning_boards = boards.iter().find(|b| b.has_won).is_none();
            if !exists_non_winning_boards {
                last_called_number = number;
                break;
            }
        }
        let winning_score = last_winning_board
            .map(|b| last_called_number * calc_score(&b))
            .unwrap();
        println!("Winning score: {}", winning_score);
    }
}

fn init_boards(board_lines: &[String]) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut rows: Vec<Vec<Entry>> = Vec::new();
    for board_line in board_lines {
        if !board_line.is_empty() {
            let row_entries = board_line
                .split(" ")
                .filter(|v| !v.is_empty())
                .map(|v| Value(v.parse::<u32>().unwrap()))
                .collect();
            rows.push(row_entries);
        }
        if rows.len() == 5 {
            boards.push(Board {
                has_won: false,
                rows,
            });
            rows = Vec::new();
        }
    }
    return boards;
}

fn mark_number(board: &mut Board, number: u32) {
    for row in board.rows.iter_mut() {
        for entry in row.iter_mut() {
            match entry {
                Value(v) => {
                    if *v == number {
                        *entry = Matched;
                    }
                }
                Entry::Matched => {}
            }
        }
    }
}

fn check_winning_board(board: &Board) -> bool {
    for row in &board.rows {
        let winning_row = row.iter().find(|e| **e != Matched).is_none();
        if winning_row {
            return true;
        }
    }
    for id in 0..5 {
        let winning_column = board
            .rows
            .iter()
            .map(|r| r[id])
            .find(|e| *e != Matched)
            .is_none();
        if winning_column {
            return true;
        }
    }
    return false;
}

fn calc_score(board: &Board) -> u32 {
    board
        .rows
        .iter()
        .flat_map(|r| r.iter())
        .map(|e| match e {
            Matched => 0,
            Value(v) => *v,
        })
        .fold(0, |agg, v| agg + v)
}
