#[derive(Clone)]
struct CrabPosition(i32);

impl CrabPosition {
    fn fuel_to_position_correct_consumption(&self, pos: i32) -> i32 {
        let dist = (self.0 - pos).abs();
        dist*(dist+1)/2
    }
}

fn parse_input(input: &str) -> Vec<CrabPosition> {
    input
        .trim()
        .split(",")
        .map(|line| CrabPosition(line.parse::<i32>().expect("input should be ints")))
        .collect()
}

fn get_optimal_position_with_old_fuel_consumption(crabs: &Vec<CrabPosition>) -> i32 {
    let mut x1 = crabs.clone();
    x1.sort_by_key(|c| c.0);

    let mid = x1.len() / 2;
    if x1.len() % 2 == 0 {
        (x1[mid - 1].0 + x1[mid].0)/2
    } else {
        x1[mid].0
    }
}

fn get_optimal_position_with_correct_fuel_consumption(crabs: &Vec<CrabPosition>) -> (i32, i32) {
    let mut optimal_position = 0;
    let mut optimal_fuel_consumption = i32::MAX;
    for possible_position in 0..crabs.iter().max_by_key(|c|c.0).expect("not empty").0 {
        let new_fuel = fuel_to_position_correct_consumption(crabs, possible_position);
        if new_fuel < optimal_fuel_consumption {
            optimal_position = possible_position;
            optimal_fuel_consumption = new_fuel;
        }
    }

    (optimal_position, optimal_fuel_consumption)
}

fn fuel_to_position_correct_consumption(crabs: &Vec<CrabPosition>, pos: i32) -> i32 {
    crabs.iter().map(|crab| crab.fuel_to_position_correct_consumption(pos)).sum()
}


fn fuel_to_position_old_consumption(crabs: &Vec<CrabPosition>, pos: i32) -> i32 {
    crabs.iter().map(|crab| (crab.0 - pos).abs()).sum()
}

fn solve_pt_1() -> i32 {
    let input = include_str!("input");
    let crabs = parse_input(input);
    fuel_to_position_old_consumption(&crabs, get_optimal_position_with_old_fuel_consumption(&crabs))
}

fn solve_pt_2() -> i32 {
    let input = include_str!("input");

    let crabs = parse_input(input);
     get_optimal_position_with_correct_fuel_consumption(&crabs).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_correct_input() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(get_optimal_position_with_old_fuel_consumption(&parse_input(input)), 2);
        assert_eq!(fuel_to_position_old_consumption(&parse_input(input), 2), 37);
        assert_eq!(get_optimal_position_with_correct_fuel_consumption(&parse_input(input)), (5,168));
    }

    #[test]
    fn should_determine_fuel_costs() {
        assert_eq!(CrabPosition(5).fuel_to_position_correct_consumption(6), 1);
        assert_eq!(CrabPosition(5).fuel_to_position_correct_consumption(3), 3);
        assert_eq!(CrabPosition(5).fuel_to_position_correct_consumption(8), 6);
    }

    #[test]
    fn print_day_7() {
        println!("We need {} fuel to reach the optimal position", solve_pt_1());
        println!("We need {} fuel to reach the optimal position with the correct crab fuel consumption", solve_pt_2());
    }
}