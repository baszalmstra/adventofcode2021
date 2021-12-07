use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let positions = std::fs::read_to_string("inputs/day7/input")?
        .split(',')
        .map(|num| -> anyhow::Result<isize> { Ok(num.parse()?) })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let (&min, &max) = positions
        .iter()
        .minmax()
        .into_option()
        .ok_or_else(|| anyhow::anyhow!("no elements"))?;

    let mut best_score = isize::MAX;
    for i in min..max {
        let score = positions
            .iter()
            .fold(0, |accum, &pos| accum + (pos - i).abs());
        best_score = best_score.min(score);
    }

    println!("Solution 1: {}", best_score);

    let mut best_score = isize::MAX;
    for i in min..max {
        let score = positions.iter().fold(0, |accum, &pos| {
            let n = (pos - i).abs();
            let c = n * (n + 1) / 2;
            accum + c
        });
        best_score = best_score.min(score);
    }

    println!("Solution 2: {}", best_score);

    Ok(())
}
