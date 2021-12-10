use crate::base::read_lines;
use crate::day4::Entry::Value;
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

#[derive(Debug, Copy, Clone)]
enum Entry {
    Value(u32),
    Matched,
}

#[derive(Debug)]
struct Board {
    id: u32,
    entries: Vec<Vec<Entry>>,
}

impl Day4Executable {
    fn process(&self, lines: Vec<String>) {
        let numbers: Vec<u32> = lines.get(0).unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect();
        let board_lines = &lines[2..];
        let mut boards = init_boards(board_lines);
        println!("{:?}", boards);
    }
}

fn init_boards(board_lines: &[String]) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut entries: Vec<Vec<Entry>> = Vec::new();
    let mut id = 1;
    for board_line in board_lines {
        if !board_line.is_empty() {
            let row_entries = board_line.split(" ").filter(|v|!v.is_empty()).map(|v| Value(v.parse::<u32>().unwrap())).collect();
            entries.push(row_entries);
        }
        if entries.len() == 5 {
            let board = Board {
                id,
                entries: entries.clone()
            };
            boards.push(board);
            id += 1;
            entries = Vec::new();
        }
    }
    return boards;
}
