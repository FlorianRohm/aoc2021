#[derive(Default, Copy, Clone, Debug)]
struct Octopus {
    level: u32,
    flashed: bool,
    primed: bool,
}

impl Octopus {
    fn new(level: u32) -> Self {
        Self {
            level,
            flashed: false,
            primed: false,
        }
    }
    fn step(mut self) -> Self {
        self.flashed = false;
        self.primed = false;
        self.level += 1;

        self
    }

    fn prime(&mut self) {
        if self.level > 9 {
            self.primed = true;
        }
    }

    fn excite(&mut self) {
        if self.flashed || self.primed {
            return;
        }

        self.level += 1
    }

    fn try_flash(&mut self) -> bool {
        if self.primed {
            self.flashed = true;
            self.primed = false;
            self.level = 0;
            true
        } else {
            false
        }
    }
}

struct Cave {
    grid: [[Octopus; 12]; 12],
    flashes: u32,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut grid = [[Octopus::default(); 12]; 12];
        let raw: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).expect("only digits in input"))
                    .collect()
            })
            .collect();

        for x in 1..11 {
            for y in 1..11 {
                grid[x][y] = Octopus::new(raw[x - 1][y - 1]);
            }
        }

        Self { grid, flashes: 0 }
    }

    fn step(&mut self) {
        self.grid = self.grid.map(|row| row.map(|octopus| octopus.step()));
    }

    fn flash(&mut self) -> bool {
        let mut flashed = false;
        for x in 1..11 {
            for y in 1..11 {
                let octopus = &mut self.grid[x][y];
                octopus.prime();
                if octopus.try_flash() {
                    flashed = true;
                    self.flashes += 1;

                    self.grid[x - 1][y - 1].excite();
                    self.grid[x - 1][y].excite();
                    self.grid[x - 1][y + 1].excite();
                    self.grid[x][y - 1].excite();
                    self.grid[x][y + 1].excite();
                    self.grid[x + 1][y - 1].excite();
                    self.grid[x + 1][y].excite();
                    self.grid[x + 1][y + 1].excite();
                }
            }
        }
        flashed
    }

    fn flash_until_settled(&mut self) -> bool {
        let mut nr_of_rounds = 0;
        while self.flash() {
            nr_of_rounds += 1;
            if nr_of_rounds > 100 {
                eprintln!("Round took more than 100 flashes");
                eprintln!("{:?}", self.grid);
                break;
            }
        }
        self.is_synchronized()
    }

    fn is_synchronized(&self) -> bool {
        self.grid.iter().as_slice()[1..11]
            .iter()
            .all(|row| {
                row.iter().as_slice()[1..11]
                    .iter()
                    .all(|octopus| octopus.level == 0)
            })
    }

    fn print(&self) -> String {
        self.grid.iter().as_slice()[1..11]
            .iter()
            .map(|row| {
                let s: String = row.iter().as_slice()[1..11]
                    .iter()
                    .map(|octopus| octopus.level.to_string())
                    .collect();
                format!("{}\n", s)
            })
            .collect()
    }
}

fn solve_pt_1() -> u32 {
    let input = include_str!("input");
    let mut cave = Cave::new(input);
    for _ in 0..100 {
        cave.step();
        cave.flash_until_settled();
    }
    cave.flashes
}
fn solve_pt_2() -> u32 {
    let input = include_str!("input");
    let mut cave = Cave::new(input);
    let mut i = 0;
    loop  {
        i += 1;
        cave.step();
        if cave.flash_until_settled() {
            return  i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_correct_board_after_step() {
        let input = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut cave = Cave::new(input);

        cave.step();
        cave.flash_until_settled();
        assert_eq!(
            cave.print(),
            "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
"
        );
        cave.step();
        cave.flash_until_settled();

        assert_eq!(
            cave.print(),
            "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
"
        )
    }

    #[test]
    fn should_calculate_the_correct_flashes_for_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut cave = Cave::new(input);
        for _ in 0..10 {
            cave.step();
            cave.flash_until_settled();
        }

        assert_eq!(cave.flashes, 204);
    }

    #[test]
    fn should_synchronize_on_step_195() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut cave = Cave::new(input);
        let mut i = 0;
        let sync = loop  {
            i += 1;
            cave.step();
            if cave.flash_until_settled() {
                break i;
            }
        };

        assert_eq!(sync,  195);
    }

    #[test]
    fn print_day_11() {
        println!("The number of flashes after 100 loops is {}", solve_pt_1());
        println!("The first synchronized flash is after {} timesteps", solve_pt_2());
    }
}
