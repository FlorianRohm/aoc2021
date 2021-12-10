use ndarray::Array2;

#[derive(Copy, Clone)]
struct MapLocation {
    height: u32,
    basin: Option<(usize, usize)>,
}

impl MapLocation {
    fn new(height: u32) -> Self {
        Self {
            height,
            basin: None,
        }
    }
}

#[derive(Clone)]
struct Grid(Array2<MapLocation>);

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let dim = (lines[0].len(), lines.len());

        let mut grid = Grid(Array2::from_shape_fn(dim, |(x, y)| {
            let x1: &str = lines[y];
            let char = x1.chars().nth(x).expect("should be in range");
            MapLocation::new(char.to_digit(10).expect("input should be parsable chars"))
        }));
        grid.find_basins();
        grid
    }

    fn find_basins(&mut self) {
        let backup = self.clone();

        self.0.indexed_iter_mut().for_each(|((x, y), value)| {
            if backup
                .get_neighbours(x, y)
                .iter()
                .all(|&neighbour| neighbour.height > value.height)
            {
                *value = MapLocation {
                    height: value.height,
                    basin: Some((x, y)),
                }
            }
        });
    }

    fn promote_basins(&mut self) {
        let backup = self.clone();

        self.0.indexed_iter_mut().for_each(|((x, y), value)| {
            if value.height == 9 {
                return;
            }

            if let Some(basin) = backup
                .get_neighbours(x, y)
                .iter()
                .flat_map(|&neighbour| neighbour.basin)
                .next()
            {
                *value = MapLocation {
                    height: value.height,
                    basin: Some(basin),
                }
            }
        });
    }

    fn full_promote_basins(&mut self) {
        let (x_max, y_max) = self.0.dim();

        for _ in 0..(x_max + y_max) {
            self.promote_basins()
        }
    }

    fn collect_basins(&self) -> Vec<((usize, usize), usize)> {
        use itertools::Itertools;
        let basin_counts = self.0.iter().flat_map(|m| m.basin).counts();

        basin_counts
            .into_iter()
            .sorted_by_key(|count| -(count.1 as isize))
            .as_slice()[0..=2]
            .to_vec()
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<MapLocation> {
        let mut neighbors: Vec<MapLocation> = Vec::new();
        let (x_max, y_max) = self.0.dim();

        if x > 0 {
            neighbors.push(self.0[[x - 1, y]])
        }
        if y > 0 {
            neighbors.push(self.0[[x, y - 1]])
        }

        if x < x_max - 1 {
            neighbors.push(self.0[[x + 1, y]])
        }
        if y < y_max - 1 {
            neighbors.push(self.0[[x, y + 1]])
        }
        neighbors
    }

    fn compute_risk_level(&self) -> u32 {
        let risk_level = self
            .0
            .iter()
            .flat_map(|value| value.basin.map(|_| value.height + 1))
            .sum();

        risk_level
    }
}

fn solve_pt_1() -> u32 {
    let input = include_str!("input");
    Grid::new(input).compute_risk_level()
}

fn solve_pt_2() -> usize {
    let input = include_str!("input");
    let mut grid = Grid::new(input);
    grid.full_promote_basins();

    grid.collect_basins().into_iter().map(|b| b.1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_test() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(Grid::new(input).compute_risk_level(), 15);
        let mut grid = Grid::new(input);
        grid.full_promote_basins();
        let basins_sizes: Vec<_> = grid.collect_basins().iter().map(|b| b.1).collect();
        assert_eq!(basins_sizes, vec![14, 9, 9]);
    }

    #[test]
    fn print_day_9() {
        println!("The cumulative risk level is {}", solve_pt_1());
        println!("The multiplied basin values are {} ", solve_pt_2());
    }
}
