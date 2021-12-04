enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Default)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

fn parse_input(lines: &str) -> Vec<Direction> {
    lines
        .trim()
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let direction = splits.next().expect("no direction");
            let amount = splits
                .next()
                .expect("no amount")
                .parse::<i32>()
                .expect("input should be ints");
            match direction {
                "forward" => Direction::Forward(amount),
                "down" => Direction::Down(amount),
                "up" => Direction::Up(amount),
                unknown => panic!("direction '{}' not found", unknown),
            }
        })
        .collect()
}

fn calculate_position(lines: &[Direction]) -> Position {
    let mut position = Position::default();
    for line in lines {
        match line {
            Direction::Forward(amount) => position.horizontal += amount,
            Direction::Up(amount) => position.depth -= amount,
            Direction::Down(amount) => position.depth += amount,
        }
    }

    position
}

fn calculate_position_with_aim(lines: &[Direction]) -> Position {
    let mut position = Position::default();
    for line in lines {
        match line {
            Direction::Forward(amount) => {
                position.horizontal += amount;
                position.depth += amount * position.aim;
            }
            Direction::Up(amount) => position.aim -= amount,
            Direction::Down(amount) => position.aim += amount,
        }
    }

    position
}

fn solve_pt_1() -> i32 {
    let input = include_str!("./input");

    let parsed = parse_input(input);
    let position = calculate_position(&parsed);

    position.depth * position.horizontal
}

fn solve_pt_2() -> i32 {
    let input = include_str!("./input");

    let parsed = parse_input(input);
    let position = calculate_position_with_aim(&parsed);

    position.depth * position.horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let parsed = parse_input(input);
        let position = calculate_position(&parsed);
        assert_eq!(position.depth, 10);
        assert_eq!(position.horizontal, 15);

        let position = calculate_position_with_aim(&parsed);
        assert_eq!(position.depth, 60);
        assert_eq!(position.horizontal, 15);
    }

    #[test]
    fn print_day2() {
        println!("position hash is {}", solve_pt_1());
        println!("position hash with correct navigation is {}", solve_pt_2());
    }
}
