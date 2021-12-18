use anyhow::anyhow;
use itertools::Itertools;
use std::collections::HashMap;

type RuleSet = HashMap<(char, char), char>;

fn parse(input: &str) -> anyhow::Result<(Vec<char>, RuleSet)> {
    let mut lines = input.lines();
    let template = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("no template"))?
        .chars()
        .collect_vec();

    lines.next().ok_or_else(|| anyhow!("missing empty line"))?;

    let mut rules = HashMap::new();
    for rule in lines {
        let (left, right) = rule
            .split_once(" -> ")
            .ok_or_else(|| anyhow!("invalid rule"))?;
        let mut left_chars = left.chars();
        let left_chars = (
            left_chars.next().ok_or_else(|| anyhow!("missing char"))?,
            left_chars.next().ok_or_else(|| anyhow!("missing char"))?,
        );
        let righ_char = right
            .chars()
            .next()
            .ok_or_else(|| anyhow!("missing right char"))?;
        rules.insert(left_chars, righ_char);
    }

    Ok((template, rules))
}

fn main() -> anyhow::Result<()> {
    let (template, rules) = parse(&std::fs::read_to_string("inputs/day14/input")?)?;

    let mut pairs = HashMap::new();
    let mut chars = template.iter().peekable();
    while let Some(&a) = chars.next() {
        if let Some(&b) = chars.peek().copied() {
            *pairs.entry((a, b)).or_insert(0) += 1;
        }
    }

    for iter in 0..40 {
        let mut new_pairs = HashMap::new();
        for ((a, b), count) in pairs {
            if let Some(&rule) = rules.get(&(a, b)) {
                *new_pairs.entry((a, rule)).or_insert(0) += count;
                *new_pairs.entry((rule, b)).or_insert(0) += count;
            } else {
                *new_pairs.entry((a, b)).or_insert(0) += count;
            }
        }
        pairs = new_pairs;

        if iter == 9 {
            println!("Solution 1: {}", get_polymer_count(&pairs, &template));
        } else if iter == 39 {
            println!("Solution 2: {}", get_polymer_count(&pairs, &template));
            break;
        }
    }

    Ok(())
}

fn get_polymer_count(pairs: &HashMap<(char, char), usize>, template: &[char]) -> usize {
    let mut char_count = HashMap::new();
    for (&(a, b), &count) in pairs {
        *char_count.entry(a).or_insert(0) += count;
        *char_count.entry(b).or_insert(0) += count;
    }
    if let Some(&first) = template.first() {
        *char_count.entry(first).or_insert(0) += 1;
    }
    if let Some(&last) = template.last() {
        *char_count.entry(last).or_insert(0) += 1;
    }

    let count = char_count
        .into_iter()
        .sorted_by_key(|(_, count)| *count)
        .collect_vec();
    count.last().unwrap().1 / 2 - count.first().unwrap().1 / 2
}
