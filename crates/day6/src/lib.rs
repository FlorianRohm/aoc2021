struct LanternFishData {
    time_until_spawn: usize,
    nr_in_bucket: usize,
}

impl LanternFishData {
    fn new(time_until_spawn: usize, nr_in_bucket: usize) -> Self {
        LanternFishData {
            time_until_spawn,
            nr_in_bucket,
        }
    }
}
struct LanternFishes(Vec<LanternFishData>);

impl LanternFishes {
    fn tick(mut self) -> Self {
        let mut new_fish_data = vec![
            LanternFishData::new(0, 0),
            LanternFishData::new(1, 0),
            LanternFishData::new(2, 0),
            LanternFishData::new(3, 0),
            LanternFishData::new(4, 0),
            LanternFishData::new(5, 0),
            LanternFishData::new(6, 0),
            LanternFishData::new(7, 0),
            LanternFishData::new(8, 0),
        ];

        self.0.into_iter().for_each(|fish| {
            if fish.time_until_spawn == 0 {
                new_fish_data[6].nr_in_bucket += fish.nr_in_bucket;
                new_fish_data[8].nr_in_bucket += fish.nr_in_bucket;
            } else {
                new_fish_data[fish.time_until_spawn - 1].nr_in_bucket += fish.nr_in_bucket;
            }
        });

        self.0 = new_fish_data;
        self
    }

    fn tick_n(mut self, n: usize) -> Self {
        for _ in 0..n {
            self = self.tick();
        }
        self
    }

    fn population(&self) -> usize {
        self.0.iter().map(|f| f.nr_in_bucket).sum()
    }
}

fn parse_inputs(input: &str) -> LanternFishes {
    let mut fishes = vec![
        LanternFishData::new(0, 0),
        LanternFishData::new(1, 0),
        LanternFishData::new(2, 0),
        LanternFishData::new(3, 0),
        LanternFishData::new(4, 0),
        LanternFishData::new(5, 0),
        LanternFishData::new(6, 0),
        LanternFishData::new(7, 0),
        LanternFishData::new(8, 0),
    ];
    input
        .trim()
        .split(",")
        .map(|line| line.parse::<usize>().expect("input should be ints"))
        .for_each(|time_to_spawn| {
            let data = fishes.get_mut(time_to_spawn).expect("just constructed");
            data.nr_in_bucket += 1;
        });
    LanternFishes(fishes)
}

fn solve_pt_1() -> usize {
    let input = include_str!("input");
    parse_inputs(input).tick_n(80).population()
}

fn solve_pt_2() -> usize {
    let input = include_str!("input");
    parse_inputs(input).tick_n(256).population()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_test_output() {
        let population = parse_inputs("3,4,3,1,2").tick_n(80).population();

        assert_eq!(population, 5934)
    }

    #[test]
    fn print_day_6() {
        println!("After 80 days, there are {} Lanternfish", solve_pt_1());
        println!("After 256 days, there are {} Lanternfish", solve_pt_2());
    }
}
