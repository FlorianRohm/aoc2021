use std::collections::HashSet;
use std::ops::Deref;

use itertools::Itertools;

use crate::numbers::PERMUTATIONS;

fn solve_pt_1() -> usize {
    let input = include_str!("input");
    let number_of_1478: usize = input
        .trim()
        .lines()
        .map(|line| {
            line.split('|')
                .nth(1)
                .expect("input should have a |")
                .split_whitespace()
                .filter(|segment| matches!(segment.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum();

    number_of_1478
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Segment {
    fn get_permuted(&self, permutation: &[Segment]) -> Segment {
        use Segment::*;
        let (pos, _) = permutation
            .iter()
            .find_position(|&s| s == self)
            .expect("could not de-permute");
        match pos {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            4 => E,
            5 => F,
            6 => G,
            _ => panic!("no permutation found"),
        }
    }
}

impl TryFrom<char> for Segment {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Segment::*;

        match value {
            'a' => Ok(A),
            'b' => Ok(B),
            'c' => Ok(C),
            'd' => Ok(D),
            'e' => Ok(E),
            'f' => Ok(F),
            'g' => Ok(G),
            _ => Err(()),
        }
    }
}

fn to_segment_set(input: &str) -> HashSet<Segment> {
    input
        .trim()
        .chars()
        .map(|c| Segment::try_from(c).expect("bad input"))
        .collect()
}

mod numbers {
    //!
    //!```text
    //! 0:      1:      2:      3:      4:
    //!  aaaa            aaaa    aaaa
    //! b    c       c       c       c  b    c
    //! b    c       c       c       c  b    c
    //!                  dddd    dddd    dddd
    //! e    f       f  e            f       f
    //! e    f       f  e            f       f
    //!  gggg            gggg    gggg
    //!
    //!   5:      6:      7:      8:      9:
    //!  aaaa    aaaa    aaaa    aaaa    aaaa
    //! b       b            c  b    c  b    c
    //! b       b            c  b    c  b    c
    //!  dddd    dddd            dddd    dddd
    //!      f  e    f       f  e    f       f
    //!      f  e    f       f  e    f       f
    //!  gggg    gggg            gggg    gggg
    //! ```

    use std::array::IntoIter;
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::Segment;

    use super::Segment::*;

    lazy_static::lazy_static! {
        pub(crate) static ref ZERO: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, B, C, E, F, G]));
        pub(crate) static ref ONE: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([C, F]));
        pub(crate) static ref TWO: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, C, D, E, G]));
        pub(crate) static ref THREE: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, C, D, F, G]));
        pub(crate) static ref FOUR: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([B, C, D, F]));
        pub(crate) static ref FIVE: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, B, D, F, G]));
        pub(crate) static ref SIX: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, B, D, E, F, G]));
        pub(crate) static ref SEVEN: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, C, F]));
        pub(crate) static ref EIGHT: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, B, C, D, E, F, G]));
        pub(crate) static ref NINE: HashSet<Segment> = HashSet::<_>::from_iter(IntoIter::new([A, B, C, D, F, G]));
        pub(crate) static ref ALL: Vec<HashSet<Segment>> = vec![ZERO.clone(), ONE.clone(), TWO.clone(), THREE.clone(), FOUR.clone(), FIVE.clone(), SIX.clone(), SEVEN.clone(), EIGHT.clone(), NINE.clone()];

        pub(crate) static ref PERMUTATIONS: Vec<Vec<Segment >> = vec![A, B, C, D, E, F, G].into_iter().permutations(7).collect();
    }
}

fn de_permutate(input: &HashSet<Segment>, permuation: &[Segment]) -> HashSet<Segment> {
    input
        .iter()
        .map(|segment| segment.get_permuted(permuation))
        .collect()
}

fn is_a_number(input: &HashSet<Segment>) -> bool {
    numbers::ALL.iter().any(|set| set == input)
}

fn translate_to_number(input: &HashSet<Segment>) -> u32 {
    use numbers::*;
    if input == ZERO.deref() {
        0
    } else if input == ONE.deref() {
        1
    } else if input == TWO.deref() {
        2
    } else if input == THREE.deref() {
        3
    } else if input == FOUR.deref() {
        4
    } else if input == FIVE.deref() {
        5
    } else if input == SIX.deref() {
        6
    } else if input == SEVEN.deref() {
        7
    } else if input == EIGHT.deref() {
        8
    } else if input == NINE.deref() {
        9
    } else {
        panic!("could not translate")
    }
}

fn solve_line(line: &str) -> u32 {
    let mut split = line.split('|');
    let input = split.next().expect("input should exist");
    let reading = split.next().expect("reading should exist");

    let input_segments: Vec<_> = parse_line_segment(input);
    let reading_segments: Vec<_> = parse_line_segment(reading);

    let correct_permutation = PERMUTATIONS
        .iter()
        .find(|permutation| {
            input_segments
                .iter()
                .all(|input_segment| is_a_number(&de_permutate(input_segment, permutation)))
        })
        .expect("no permutation found");

    let reading_output = reading_segments
        .iter()
        .map(|segment| de_permutate(segment, correct_permutation))
        .map(|segment| translate_to_number(&segment))
        .fold(0, |acc, elem| acc * 10 + elem);
    reading_output
}

fn parse_line_segment(input: &str) -> Vec<HashSet<Segment>> {
    input
        .trim()
        .split_whitespace()
        .map(to_segment_set)
        .collect()
}

fn solve_pt_2() -> u32 {
    let input = include_str!("input");
    input.trim().lines().map(solve_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init() {
        assert_eq!(numbers::PERMUTATIONS.len(), 5040); // 7 factorial
    }

    #[test]
    fn should_solve_a_line() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solve_line(input), 5353);
    }

    #[test]
    fn print_day_8() {
        println!("The output has {} 1 4 7 8 characters in it", solve_pt_1());
        println!("The accumulated output values are {} ", solve_pt_2());
    }
}
