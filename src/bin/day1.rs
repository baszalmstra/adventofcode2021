fn main() -> anyhow::Result<()> {
    let depths: Vec<usize> = std::fs::read_to_string("inputs/day1/input")?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }

    println!("Solution 1: {}", count);

    let mut count = 0;
    let mut previous_sum = None;
    for i in 0..depths.len() - 2 {
        let sum = depths[i] + depths[i + 1] + depths[i + 2];
        match previous_sum {
            None => {}
            Some(previous_sum) => {
                if previous_sum < sum {
                    count += 1;
                }
            }
        }
        previous_sum = Some(sum);
    }

    println!("Solution 2: {}", count);

    Ok(())
}
