#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    fn corruption_score(&self) -> u64 {
        match self {
            Bracket::Round => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137
        }
    }

    fn completion_score(&self) -> u64 {
        match self {
            Bracket::Round => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tokens {
    Open(Bracket),
    Close(Bracket),
}

impl From<char> for Tokens {

    fn from(value: char) -> Self {
        use Tokens::*;
        use Bracket::*;
        match value {
            '(' => Open(Round),
            '[' => Open(Square),
            '{' => Open(Curly),
            '<' => Open(Angle),
            ')' => Close(Round),
            ']' => Close(Square),
            '}' => Close(Curly),
            '>' => Close(Angle),
                _ => unreachable!("level input only has brackets")
        }
    }
}


struct Chunk(Vec<Tokens>);

impl From<String> for Chunk {
    fn from(input: String) -> Self {
        Self(input.chars().map(|char| char.into()).collect())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    Corrupted(Bracket),
    Incomplete(Vec<Bracket>),
    Broken
}

impl ParseError {
    fn corruption_points(&self) -> u64 {
        match self {
            ParseError::Corrupted(bracket) => bracket.corruption_score(),
            _ => 0
        }
    }
    fn completion_points(&self) -> Option<u64> {
        match self {
            ParseError::Incomplete(brackets) => {
                Some(brackets.iter().rfold(0, |acc, bracket| acc*5 + bracket.completion_score()))
            }
            _ => None
        }
    }
}

impl Chunk {
    fn parse(&self) -> Result<(), ParseError> {
        let mut stack: Vec<Bracket> = vec![];
        for bracket in &self.0 {
            match bracket {
                Tokens::Open(bracket) => stack.push(*bracket),
                Tokens::Close(bracket) => {
                    if *bracket != stack.pop().ok_or(ParseError::Broken)? {
                        return Err(ParseError::Corrupted(*bracket))
                    }
                }
            }
        }

        if !stack.is_empty() {
            Err(ParseError::Incomplete(stack))
        } else {
            Ok(())
        }
    }
}


fn solve_pt_1() -> u64 {
    let input = include_str!("input");
    solve_pt_1_with_input(input)
}

fn solve_pt_1_with_input(input: &str) -> u64 {
    parse_to_chunks(input).iter().flat_map(|chunk| chunk.parse().err()).map(|parse_error| parse_error.corruption_points()).sum()
}

fn solve_pt_2_with_input(input: &str) -> u64 {
    use itertools::Itertools;
    let iter: Vec<_> = parse_to_chunks(input).iter().flat_map(|chunk| chunk.parse().err()).flat_map(|parse_error| parse_error.completion_points()).sorted().collect();

    iter[iter.len()/2]
}

fn parse_to_chunks(input: &str) -> Vec<Chunk> {
    input.trim().lines().map(|line| Chunk::from(line.to_string())).collect()
}

fn solve_pt_2() -> u64 {
    let input = include_str!("input");
    solve_pt_2_with_input(input)
}

#[cfg(test)]
mod tests {
    use crate::Bracket::*;
    use crate::Tokens::*;
    use super::*;

    #[test]
    fn chunk_parsing() {
        assert_eq!(Chunk(vec![Open(Round), Close(Round)]).parse(), Ok(()));
        assert_eq!(Chunk(vec![Open(Round), Close(Angle)]).parse(), Err(ParseError::Corrupted(Angle)));
        assert_eq!(Chunk::from("([])".to_string()).parse(), Ok(()));
        assert_eq!(Chunk::from("{()()()}".to_string()).parse(), Ok(()));
        assert_eq!(Chunk::from("[<>({}){}[([])<>]]".to_string()).parse(), Ok(()));
        assert_eq!(Chunk::from("{([(<{}[<>[]}>{[]{[(<()>".to_string()).parse(), Err(ParseError::Corrupted(Curly)));
        assert_eq!(Chunk::from("[({(<(())[]>[[{[]{<()<>>".to_string()).parse(), Err(ParseError::Incomplete(vec![Curly, Curly, Square, Square, Round, Curly, Round, Square])));
    }

    #[test]
    fn completion_score() {
        assert_eq!(ParseError::Incomplete(vec![Angle, Curly, Round, Square]).completion_points().expect("works"), 294);
    }

    #[test]
    fn test_input() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve_pt_1_with_input(input), 26397);
        assert_eq!(solve_pt_2_with_input(input), 288957);
    }


    #[test]
    fn print_day_10() {
        println!("The total syntax error score is {}", solve_pt_1());
        println!("The middle score for the completions is {} ", solve_pt_2());
    }
}
