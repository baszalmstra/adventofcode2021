use itertools::Itertools;

fn corrupted_line_score(line: &str) -> Option<usize> {
    let mut parse_stack = Vec::new();
    for c in line.chars() {
        match c {
            c if Some(&c) == parse_stack.last() => {
                parse_stack.pop();
            }
            '(' => parse_stack.push(')'),
            '[' => parse_stack.push(']'),
            '{' => parse_stack.push('}'),
            '<' => parse_stack.push('>'),
            ')' => return Some(3),
            ']' => return Some(57),
            '}' => return Some(1197),
            '>' => return Some(25137),
            _ => unreachable!(),
        }
    }

    None
}

fn completion_line_score(line: &str) -> Option<usize> {
    let mut parse_stack = Vec::new();
    for c in line.chars() {
        match c {
            c if Some(&c) == parse_stack.last() => {
                parse_stack.pop();
            }
            '(' => parse_stack.push(')'),
            '[' => parse_stack.push(']'),
            '{' => parse_stack.push('}'),
            '<' => parse_stack.push('>'),
            ')' | ']' | '}' | '>' => return None,
            _ => unreachable!(),
        }
    }

    Some(parse_stack.into_iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    }))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day10/input")?;

    let corrupted_score: usize = input.lines().filter_map(corrupted_line_score).sum();
    println!("Solution 1: {}", corrupted_score);

    let completion_scores = input
        .lines()
        .filter_map(completion_line_score)
        .sorted_unstable()
        .collect_vec();
    let completion_score = completion_scores[completion_scores.len() / 2];
    println!("Solution 2: {}", completion_score);

    Ok(())
}
