use itertools::Itertools;

struct Entry {
    signals: Vec<String>,
    digits: Vec<String>,
}

fn parse(input: &str) -> anyhow::Result<Vec<Entry>> {
    input
        .lines()
        .map(|line| -> anyhow::Result<Entry> {
            let (left, right) = line
                .split_once(" | ")
                .ok_or(anyhow::anyhow!("no delimiter found"))?;
            Ok(Entry {
                signals: left
                    .split_whitespace()
                    .map(|block| block.to_owned())
                    .collect(),
                digits: right
                    .split_whitespace()
                    .map(|block| block.to_owned())
                    .collect(),
            })
        })
        .collect()
}

fn map_chars_to_digit(mapping: &[char], signal: &str) -> Option<usize> {
    let decoded = signal
        .chars()
        .map(|c| mapping[(c as u8 - b'a') as usize])
        .sorted()
        .collect::<String>();
    match decoded.as_str() {
        "abcdeg" => Some(0),
        "ab" => Some(1),
        "acdfg" => Some(2),
        "abcdf" => Some(3),
        "abef" => Some(4),
        "bcdef" => Some(5),
        "bcdefg" => Some(6),
        "abd" => Some(7),
        "abcdefg" => Some(8),
        "abcdef" => Some(9),
        _ => None,
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day8/input")?;
    let entries = parse(&input)?;

    let odds: usize = entries
        .iter()
        .map(|entry| {
            entry
                .digits
                .iter()
                .filter(|block| {
                    block.len() == 2 || block.len() == 4 || block.len() == 3 || block.len() == 7
                })
                .count()
        })
        .sum();
    println!("Solution 1: {}", odds);

    let result = entries
        .iter()
        .map(|entry| {
            "abcdefg"
                .chars()
                .permutations(7)
                .find(|perm| {
                    entry
                        .signals
                        .iter()
                        .map(|s| map_chars_to_digit(perm, s))
                        .all(|o| o.is_some())
                })
                .map(|perm| {
                    entry
                        .digits
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, s)| map_chars_to_digit(&perm, s).unwrap() * 10usize.pow(i as u32))
                        .sum::<usize>()
                })
                .unwrap()
        })
        .sum::<usize>();

    println!("Solution 2: {}", result);

    Ok(())
}
