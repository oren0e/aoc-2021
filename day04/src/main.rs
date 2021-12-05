use std::collections::HashMap;
use std::fs;

type Board = Vec<Vec<String>>;

fn main() {
    // read the data
    let data = fs::read_to_string("data/real01.txt").expect("Unable to read file");
    let split = data.split("\n\n");
    let raw_input: Vec<String> = split.map(|x| x.to_string()).collect();
    let numbers: Vec<String> = raw_input[0].split(",").map(|x| x.to_string()).collect();
    let boards = get_boards_from_input(raw_input[1..].to_vec());
    println!("{:?}", run_game_on_boards(boards.clone(), numbers.clone()));
    println!("{:?}", run_game_part2_on_boards(boards.clone(), numbers));
}

fn parse_board(raw_string: String) -> Board {
    let board_help: Vec<String> = raw_string.split("\n").map(|x| x.to_string()).collect();
    let mut board = vec![];
    for row in board_help {
        let mut parsed_row: Vec<String> = row.split(" ").map(|x| x.to_string()).collect();
        parsed_row.retain(|x| *x != "".to_string());
        board.push(parsed_row);
    }
    board
}

fn get_boards_from_input(input: Vec<String>) -> Vec<Board> {
    let mut boards = vec![];
    for item in input {
        boards.push(parse_board(item));
    }
    boards
}

fn transpose_board(board: Board) -> Board {
    let mut new_board = vec![];
    for d in 0..board.first().expect("empty board").len() {
        let mut new_row = vec![];
        for row in board.iter() {
            for (j, col) in row.iter().enumerate() {
                if d == j {
                    new_row.push(col.to_string())
                }
            }
        }
        new_board.push(new_row);
    }
    new_board
}

fn run_game_on_boards(mut boards: Vec<Board>, numbers: Vec<String>) -> usize {
    // replaces matching numbers on board with 'x'
    // returns the index of the winning board
    for number in numbers {
        for b in 0..boards.len() {
            for i in 0..boards[b].clone().len() {
                for j in 0..boards[b][i].len() {
                    if boards[b][i][j] == number {
                        boards[b][i][j] = "x".to_string();
                    }
                }
                if boards[b][i].iter().all(|x| *x == "x".to_string()) {
                    let parsed_number = number.parse::<i32>().expect("parsing error");
                    println!(
                        "Winning score: {:?}",
                        get_winning_board_score(boards[b].clone(), parsed_number)
                    );
                    return b;
                }
            }
            let board_t = transpose_board(boards[b].clone());
            for row_t in board_t {
                if row_t.iter().all(|x| *x == "x".to_string()) {
                    return b;
                }
            }
        }
    }
    panic!("No winners found!");
}

fn run_game_part2_on_boards(mut boards: Vec<Board>, numbers: Vec<String>) -> usize {
    // replaces matching numbers on board with 'x'
    // returns the index of the winning board
    let mut last_winner: Option<(usize, i32)> = None;
    let mut winning_boards: HashMap<usize, Vec<i32>> = HashMap::new();
    for number in numbers {
        for b in 0..boards.len() {
            for i in 0..boards[b].clone().len() {
                for j in 0..boards[b][i].len() {
                    if boards[b][i][j] == number {
                        boards[b][i][j] = "x".to_string();
                    }
                }
                if boards[b][i].iter().all(|x| *x == "x".to_string()) {
                    let parsed_number = number.parse::<i32>().expect("parsing error");
                    if winning_boards.len() < boards.len() {
                        winning_boards
                            .entry(b)
                            .and_modify(|e| e.push(parsed_number))
                            .or_insert_with(|| vec![parsed_number]);
                    } else {
                        if let Some((k, v)) = find_min_length_vec_in_map(&winning_boards) {
                            println!("{:?}", get_winning_board_score(boards[*k].clone(), v));
                        }
                        last_winner = Some((b, parsed_number));
                        println!(
                            "The last winning board score: {:?}",
                            get_winning_board_score(
                                boards[last_winner.expect("Option is None").0].clone(),
                                last_winner.expect("Option is None").1
                            )
                        );
                        return last_winner.expect("Option is None").0;
                    }
                }
            }
            let board_t = transpose_board(boards[b].clone());
            for row_t in board_t {
                if row_t.iter().all(|x| *x == "x".to_string()) {
                    let parsed_number = number.parse::<i32>().expect("parsing error");
                    if winning_boards.len() < boards.len() {
                        winning_boards
                            .entry(b)
                            .and_modify(|e| e.push(parsed_number))
                            .or_insert_with(|| vec![parsed_number]);
                    } else {
                        last_winner = Some((b, parsed_number));
                        println!(
                            "The last winning board score (vertical): {:?}",
                            get_winning_board_score(
                                boards[last_winner.expect("Option is None").0].clone(),
                                last_winner.expect("Option is None").1
                            )
                        );
                        return last_winner.expect("Option is None").0;
                    }
                }
            }
        }
    }
    panic!("no winners found!");
}

fn get_winning_board_score(board: Board, last_number_called: i32) -> i32 {
    let mut sum_non_marked = 0;
    for row in board {
        for item in row {
            if item != "x".to_string() {
                sum_non_marked += item.parse::<i32>().expect("parsing error");
            }
        }
    }
    sum_non_marked * last_number_called
}

fn find_min_length_vec_in_map(hash_map: &HashMap<usize, Vec<i32>>) -> Option<(&usize, i32)> {
    hash_map
        .iter()
        .min_by(|a, b| a.1.len().cmp(&b.1.len()))
        .map(|(k, v)| (k, v[0]))
}
