use itertools::Itertools;
use std::collections::VecDeque;

type Number = VecDeque<(u8, u8)>;

fn parse_number(line: &str) -> Number {
    line.chars()
        .fold(
            (0, VecDeque::with_capacity(line.len() / 2)),
            |(mut depth, mut vec), c| {
                match c {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    '0'..='9' => vec.push_back((depth, c.to_digit(10).unwrap() as u8)),
                    _ => {}
                }
                (depth, vec)
            },
        )
        .1
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day18/input")?;
    let numbers = input.lines().map(parse_number).collect_vec();

    let result = numbers
        .iter()
        .cloned()
        .fold1(|mut init, mut value| {
            add(&mut init, &mut value);
            reduce(&mut init, 0);
            init
        })
        .unwrap();

    println!("Solution 1: {:?}", magnitude(&result, &mut 0, 1));

    let max_magnitude = numbers
        .into_iter()
        .tuple_combinations()
        .flat_map(|(a, b)| [(a.clone(), b.clone()), (b, a)])
        .map(|(mut a, mut b)| {
            add(&mut a, &mut b);
            reduce(&mut a, 0);
            magnitude(&a, &mut 0, 1)
        })
        .max()
        .unwrap();

    println!("Solution 2: {:?}", max_magnitude);

    Ok(())
}

fn add(number: &mut Number, other: &mut Number) {
    number.append(other);
    number.iter_mut().for_each(|(depth, _)| *depth += 1);
}

fn reduce(number: &mut Number, start_idx: usize) {
    for i in start_idx..number.len() - 1 {
        if number[i].0 == 5 {
            let (left, right) = (number[i].1, number[i + 1].1);
            number[i] = (4, 0);
            number.remove(i + 1);
            let _ = number.get_mut(i.overflowing_sub(1).0).map(|n| n.1 += left);
            let _ = number.get_mut(i + 1).map(|n| n.1 += right);
            return reduce(number, i);
        }
    }
    for i in 0..number.len() {
        let (depth, value) = number[i];
        if value >= 10 {
            number[i] = (depth + 1, value / 2);
            number.insert(i + 1, (depth + 1, (value + 1) / 2));
            return reduce(number, i);
        }
    }
}

fn magnitude(number: &Number, index: &mut usize, depth: u8) -> usize {
    let left = 3 * if number[*index].0 == depth {
        *index += 1;
        number[*index - 1].1 as usize
    } else {
        magnitude(number, index, depth + 1)
    };
    let right = 2 * if number[*index].0 == depth {
        *index += 1;
        number[*index - 1].1 as usize
    } else {
        magnitude(number, index, depth + 1)
    };

    left + right
}
