
fn parse_input(lines: &str) -> Vec<Vec<u32>> {
    lines.trim().lines().map(|line|
        line.chars().map(|char| char.to_string().parse::<u32>().expect("parsing should work")).collect::<Vec<_>>()
    ).collect()
}

fn get_gamma_epsilon(input: Vec<Vec<u32>>) -> (u32, u32) {
    let len = input.len() as u32;
    let mut epsilon = 0;
    let mut gamma = 0;
    input.into_iter()
        .reduce(|orig, new| {
            orig.iter().zip(new.iter()).map(|(a, b)| a + b).collect()
        }).expect("vector should have elements to reduce")
        .iter().map(|a| { if a * 2 > len { 1 } else { 0 } })
        .rev()
        .enumerate().for_each(|(index, bit)| {
        gamma += 2u32.pow(index as u32) * bit;
        epsilon += 2u32.pow(index as u32) * (1 - bit);
    });

    (gamma, epsilon)
}

fn get_life_support_read(input: Vec<Vec<u32>>, determine_keeper: fn(u32, u32) -> u32) -> Vec<u32> {
    let len = input.len();
    let mut input = input;

    for index in 0..len {
        let items: u32 = input.iter().map(|v| v.get(index).expect("should be in range")).sum();

        let dominant = determine_keeper(input.len() as u32, items);

        input = input.into_iter().filter(|vec| {
            *vec.get(index).expect("should be in range") == dominant
        }).collect();
        if input.len() == 1 {
            return input.get(0).unwrap().to_owned();
        }
    }

    panic!("filtering was not enough...")
}

fn determine_keeper_oxygen(len: u32, items: u32) -> u32 {
    if items * 2 >= len { 1 } else { 0 }
}

fn determine_keeper_co2(len: u32, items: u32) -> u32 {
    if items * 2 >= len { 0 } else { 1 }
}

fn convert_to_decimal(input: Vec<u32>) -> u32 {
    let mut res = 0;
    input.iter().rev()
        .enumerate().for_each(|(index, bit)| {
        res += 2u32.pow(index as u32) * bit;
    });
    res
}

fn solve_pt_1() -> u32 {
    let input = include_str!("./input");

    let parsed = parse_input(input);
    let (gamma, epsilon) = get_gamma_epsilon(parsed);

    gamma * epsilon
}

fn solve_pt_2() -> u32 {
    let input = include_str!("./input");

    let parsed = parse_input(input);
    let oxygen = convert_to_decimal(get_life_support_read(parsed.clone(), determine_keeper_oxygen));
    let co2 = convert_to_decimal(get_life_support_read(parsed.clone(), determine_keeper_co2));

    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let parsed = parse_input(input);
        let (gamma, epsilon) = get_gamma_epsilon(parsed.clone());
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);

        let oxygen = convert_to_decimal(get_life_support_read(parsed.clone(), determine_keeper_oxygen));

        assert_eq!(oxygen, 23);

        let co2 = convert_to_decimal(get_life_support_read(parsed.clone(), determine_keeper_co2));

        assert_eq!(co2, 10);
    }

    #[test]
    fn test_convert_to_decimal() {
        assert_eq!(convert_to_decimal(vec![1,1,1,1]), 15);
        assert_eq!(convert_to_decimal(vec![0,1,1,0]), 6);
    }

    #[test]
    fn print_day2() {
        println!("The power consumption is {}", solve_pt_1());
        println!("The life support readings are {}", solve_pt_2());
    }
}
