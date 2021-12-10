use ndarray::{s, Array2};

struct Board(Array2<i32>);

impl Board {
    fn mark_horizontal(&mut self, line: &HorizontalLine) {
        self.0
            .slice_mut(s!(line.start_x..=line.end_x, line.y))
            .map_inplace(|field| *field += 1);
    }

    fn mark_vertical(&mut self, line: &VerticalLine) {
        self.0
            .slice_mut(s!(line.x, line.start_y..=line.end_y))
            .map_inplace(|field| *field += 1);
    }
    fn mark_diagonal(&mut self, line: &DiagonalLine) {
        for increase in 0..=line.length {
            let y_increase = if line.upward_sloping { increase } else { -increase };
            let x = (line.start_x + increase) as usize;
            let y = (line.start_y + y_increase) as usize;
            self.0[(x, y)] += 1;
        }
    }

    fn count_intersections(&self) -> usize {
        self.0.iter().filter(|&&field| field >= 2).count()
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

impl Line {
    fn new(start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> Line {
        Line {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
    fn to_vertical(self) -> Option<VerticalLine> {
        let mut y = vec![self.start_y, self.end_y];
        y.sort_unstable();
        if self.start_x == self.end_x {
            Some(VerticalLine {
                x: self.start_x,
                start_y: y[0],
                end_y: y[1],
            })
        } else {
            None
        }
    }

    fn to_horizontal(self) -> Option<HorizontalLine> {
        let mut x = vec![self.start_x, self.end_x];
        x.sort_unstable();
        if self.start_y == self.end_y {
            Some(HorizontalLine {
                y: self.start_y,
                start_x: x[0],
                end_x: x[1],
            })
        } else {
            None
        }
    }

    fn to_diagonal(self) -> Option<DiagonalLine> {
        let length_x = self.end_x - self.start_x;
        let length_y = self.end_y - self.start_y;

        if length_x == length_y {
            Some(DiagonalLine {
                start_x: self.start_x.min(self.end_x),
                start_y: self.start_y.min(self.end_y),
                length: length_x.abs(),
                upward_sloping: true,
            })
        } else if length_x == -length_y {
            let (start_x, start_y) = if self.start_x < self.end_x {
                (self.start_x, self.start_y)
            } else {
                (self.end_x, self.end_y)
            };

            Some(DiagonalLine {
                start_x,
                start_y,
                length: length_x.abs(),
                upward_sloping: false,
            })
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct VerticalLine {
    x: i32,
    start_y: i32,
    end_y: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct HorizontalLine {
    y: i32,
    start_x: i32,
    end_x: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct DiagonalLine {
    start_x: i32,
    start_y: i32,
    length: i32,
    upward_sloping: bool,
}

fn parse_line(s: &str) -> anyhow::Result<Line> {
    peg::parser! {
          grammar parser() for str {
                rule number() -> i32
                  = n:$(['0'..='9']+) { n.parse().unwrap() }

                pub(crate) rule line() -> Line
                    = start_x:number() "," start_y:number() " -> " end_x:number() "," end_y:number() {
                    Line::new(start_x, start_y, end_x, end_y )
                    }
        }
    }

    Ok(parser::line(s)?)
}

fn solve_pt_1() -> usize {
    let input = include_str!("input");
    let (lines, mut board) = parse(input);

    add_straight_intersections(lines, &mut board);
    board.count_intersections()
}

fn solve_pt_2() -> usize {
    let input = include_str!("input");
    let (lines, mut board) = parse(input);

    add_straight_intersections(lines.clone(), &mut board);
    add_diagonal_intersections(lines, &mut board);
    board.count_intersections()
}

fn parse(input: &str) -> (Vec<Line>, Board) {
    use ndarray::prelude::*;
    let lines: Vec<_> = input
        .trim()
        .lines()
        .map(|line| parse_line(line).expect("parsing failed"))
        .collect();
    let max_x = lines
        .iter()
        .max_by_key(|l| l.end_x.max(l.start_x))
        .map(|line| line.end_x.max(line.start_x))
        .expect("we have a line") as usize;
    let max_y = lines
        .iter()
        .max_by_key(|l| l.end_y.max(l.start_y))
        .map(|line| line.end_y.max(line.start_y))
        .expect("we have a line") as usize;

    (
        lines,
        Board(Array2::default((max_x + 1, max_y + 1).into_shape())),
    )
}

fn add_straight_intersections(lines: Vec<Line>, board: &mut Board) {
    let horizontal_lines: Vec<_> = lines
        .clone()
        .into_iter()
        .filter_map(Line::to_horizontal)
        .collect();
    let vertical_lines: Vec<_> = lines.into_iter().filter_map(Line::to_vertical).collect();

    for horizontal in &horizontal_lines {
        board.mark_horizontal(horizontal);
    }
    for vertical_line in &vertical_lines {
        board.mark_vertical(vertical_line);
    }
}

fn add_diagonal_intersections(lines: Vec<Line>, board: &mut Board) {
    let diags: Vec<_> = lines.into_iter().filter_map(Line::to_diagonal).collect();

    for diag in &diags {
        board.mark_diagonal(diag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_line() {
        assert_eq!(
            parse_line("55,9 -> 0,9").unwrap(),
            Line {
                start_x: 0,
                start_y: 9,
                end_x: 55,
                end_y: 9
            }
        )
    }

    #[test]
    fn should_solve_part_one() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let (lines, mut board) = parse(input);
        add_straight_intersections(lines.clone(), &mut board);
        let intersections = board.count_intersections();

        assert_eq!(intersections, 5);
        add_diagonal_intersections(lines, &mut board);
        let intersections = board.count_intersections();

        assert_eq!(intersections, 12);
    }

    #[test]
    fn print_day_5() {
        println!("The number of overlaps is {}", solve_pt_1());
        println!("The total number of overlaps with diagonals is {}", solve_pt_2());
    }
}
