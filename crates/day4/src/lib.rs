use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty(u32),
    Marked(u32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Board([[Cell; 5]; 5]);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct WinningData {
    final_draw: u32,
    empty_cells: u32,
    winning_move: usize,
}

impl Board {
    fn mark_number(&mut self, input: u32) {
        self.0 = self.0.map(|row| {
            row.map(|cell| match cell {
                Cell::Empty(val) => {
                    if val == input {
                        Cell::Marked(val)
                    } else {
                        Cell::Empty(val)
                    }
                }
                cell => cell,
            })
        });
    }

    fn has_won(&self) -> bool {
        let row_win = self
            .0
            .iter()
            .any(|row| row.iter().all(|cell| matches!(cell, Cell::Marked(_))));
        if row_win {
            return true;
        }

        for i in 0..5 {
            let column_win = vec![
                self.0[0][i],
                self.0[1][i],
                self.0[2][i],
                self.0[3][i],
                self.0[4][i],
            ]
            .iter()
            .all(|cell| matches!(cell, Cell::Marked(_)));
            if column_win {
                return true;
            }
        }

        false
    }

    fn count_unmarked(&self) -> u32 {
        self.0
            .iter()
            .map::<u32, _>(|row| {
                row.iter()
                    .filter_map(|c| match c {
                        Cell::Empty(val) => Some(val),
                        Cell::Marked(_) => None,
                    })
                    .sum()
            })
            .sum()
    }

    fn play_until_win(&mut self, input: &[u32]) -> Option<WinningData> {
        for (index, &draw) in input.iter().enumerate() {
            self.mark_number(draw);
            if self.has_won() {
                return Some(WinningData {
                    final_draw: draw,
                    empty_cells: self.count_unmarked(),
                    winning_move: index,
                });
            }
        }
        None
    }
}

fn parse_inputs(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(",")
        .map(|line| line.parse::<u32>().expect("input should be ints"))
        .collect()
}

fn parse_boards(input: &str) -> Vec<Board> {
    input
        .trim()
        .split("\n\n")
        .map(|board| {
            Board(
                board
                    .trim()
                    .lines()
                    .into_iter()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|item| {
                                Cell::Empty(item.parse::<u32>().expect("input should be ints"))
                            })
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}

fn solve_pt_1() -> u32 {
    let input = include_str!("./inputs");
    let inputs = parse_inputs(input);
    let mut boards = parse_boards(include_str!("./boards"));

    for draw in inputs {
        for board in &mut boards {
            board.mark_number(draw);
            if board.has_won() {
                return board.count_unmarked() * draw;
            }
        }
    }

    panic!("nobody won :(");
}

fn solve_pt_2() -> u32 {
    let inputs = parse_inputs(include_str!("./inputs"));
    let boards = parse_boards(include_str!("./boards"));

    let winning_board_data = boards
        .into_iter()
        .filter_map(|mut board| board.play_until_win(&inputs))
        .sorted_by_key(|w| w.winning_move)
        .last()
        .unwrap();


    winning_board_data.final_draw * winning_board_data.empty_cells
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning() {
        use Cell::*;
        let mut board = Board([
            [Empty(14), Empty(21), Empty(17), Empty(24), Empty(4)],
            [Empty(10), Empty(16), Empty(15), Empty(9), Empty(19)],
            [Empty(18), Empty(8), Empty(23), Empty(26), Empty(20)],
            [Empty(22), Empty(11), Empty(13), Empty(6), Empty(5)],
            [Empty(2), Empty(0), Empty(12), Empty(3), Empty(7)],
        ]);

        for draw in vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21] {
            board.mark_number(draw);
            assert_eq!(board.has_won(), false);
        }
        board.mark_number(24);
        assert!(board.has_won());
        let unmarked = board.count_unmarked();

        assert_eq!(unmarked, 188)
    }

    #[test]
    fn print_day_4() {
        println!("The final score on the winning board is {}", solve_pt_1());
        println!("The final score of the last board to win is {}", solve_pt_2());
    }
}
