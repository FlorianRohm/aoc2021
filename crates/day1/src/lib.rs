fn depth_increases(lines: &[i32]) -> usize {
    lines
        .windows(2)
        .filter(|&window| window[1] > window[0])
        .count()
}

fn depth_increases_triple(lines: &[i32]) -> usize {
    lines
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|&window| window[1] > window[0])
        .count()
}

fn solve_pt_1() -> usize {
    let input = include_str!("./input");
    let lines: Vec<i32> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().expect("input should be ints"))
        .collect();

    depth_increases(&lines)
}
fn solve_pt_2() -> usize {
    let input = include_str!("./input");
    let lines: Vec<i32> = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().expect("input should be ints"))
        .collect();

    depth_increases_triple(&lines)
}

#[cfg(test)]
mod tests {
    use crate::{depth_increases, depth_increases_triple, solve_pt_1, solve_pt_2};

    #[test]
    fn it_works() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(depth_increases(&input), 7);
        assert_eq!(depth_increases_triple(&input), 5);
    }

    #[test]
    fn print_day1() {
        println!("depth inceases {} times", solve_pt_1());
        println!("depth triple increase {} times", solve_pt_2());
    }
}
