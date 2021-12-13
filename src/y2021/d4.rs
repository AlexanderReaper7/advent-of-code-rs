//! --- Day 4: Giant Squid ---

struct Board {
    numbers: Vec<Vec<u32>>, // [row][col] = n
    marked: Vec<Vec<bool>>,
}
impl Board {
    fn new(numbers: Vec<Vec<u32>>) -> Self { 
        Board {
        marked: vec![vec![false; numbers[0].len()]; numbers.len()],
        numbers,
        } 
    }

    fn check_bingo(&self, row: usize, col: usize) -> bool {
        // check horizontal
        let mut i = self.marked[row].len();
        for nums in &self.marked[row] {
            if !*nums {
                break;
            }
            i -= 1;
            if i == 0 {
                return true;
            }
        }
        // check vertical
        i = self.marked.len();
        for nums in &self.marked {
            if !nums[col] {
                break;
            }
            i -= 1;
            if i == 0 {
                return true;
            }
        }
        false
    }
}

pub fn part1(input: String) -> String {
    // first line is the called numbers separated by commas
    // parse called numbers
    let calls = input.lines().next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    // parse boards
    let boards_input: Vec<&str> = input.lines().skip(1).collect();
    let mut boards = boards_input
        // split into individual boards by empty lines and remove any remaining empty lines
        .split(|s| s.is_empty()).filter(|s| !s.is_empty())
        // split into numbers
        .map(|board| board.into_iter().map(|line| line.split_whitespace()
        // split and parse into numbers as 2d array
        .map(|numbers| numbers.parse::<u32>().unwrap())
        .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>())
        // convert into Boards
        .map(|board| Board::new(board))
        .collect::<Vec<Board>>();

    for call in calls {
        for board in &mut boards {
            for row in 0..board.numbers.len() {
                for col in 0..board.numbers[row].len() {
                    if board.numbers[row][col] == call {
                        board.marked[row][col] = true;
                        if board.check_bingo(row, col) {
                            // sum unmarked numbers
                            let sum = board.numbers.iter().enumerate().map(|(i, inner_row)| inner_row.iter().enumerate().filter(|(j, inner_col)| !board.marked[i][*j]).map(|u|u.1).sum::<u32>()).sum::<u32>();
                            // multiply by call
                            let score = call * sum;
                            // return result
                            return score.to_string();
                        }
                    }
                }
            }
        }
    }
    panic!("No bingo found");
}

pub fn part2(input: String) -> String {
    // first line is the called numbers separated by commas
    // parse called numbers
    let calls = input.lines().next().unwrap().split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    // parse boards
    let boards_input: Vec<&str> = input.lines().skip(1).collect();
    let mut boards = boards_input
        // split into individual boards by empty lines and remove any remaining empty lines
        .split(|s| s.is_empty()).filter(|s| !s.is_empty())
        // split into numbers
        .map(|board| board.into_iter().map(|line| line.split_whitespace()
        // split and parse into numbers as 2d array
        .map(|numbers| numbers.parse::<u32>().unwrap())
        .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>())
        // convert into Boards
        .map(|board| Board::new(board))
        .collect::<Vec<Board>>();

    for call in calls {
        let mut board = boards.len();
        'board_loop: while let Some(_) = board.checked_sub(1) { // breaks on checked sub that underflows
            board -= 1;
            for row in 0..boards[board].numbers.len() {
                for col in 0..boards[board].numbers[row].len() {
                    if boards[board].numbers[row][col] == call {
                        boards[board].marked[row][col] = true;
                        if boards[board].check_bingo(row, col) {
                            if boards.len() > 1 {
                                boards.remove(board);
                                continue 'board_loop;
                            } else {
                                // sum unmarked numbers
                                let sum = boards[board].numbers.iter().enumerate().map(|(i, inner_row)| inner_row.iter().enumerate().filter(|(j, inner_col)| !boards[board].marked[i][*j]).map(|u|u.1).sum::<u32>()).sum::<u32>();
                                // multiply by call
                                let score = call * sum;
                                // return result
                                return score.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("No bingo found");
}
