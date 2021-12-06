fn main() -> anyhow::Result<()> {
    let days = std::fs::read_to_string("inputs/day6/input")?
        .split(',')
        .map(|num| -> anyhow::Result<usize> { Ok(num.parse()?) })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut mod_buf = [0usize; 9];
    for day in days {
        mod_buf[day] += 1;
    }

    for day in 0..256 {
        mod_buf[(day + 7) % 9] += mod_buf[day % 9];

        if day == 79 {
            println!("Solution 1: {}", mod_buf.iter().sum::<usize>());
        } else if day == 255 {
            println!("Solution 2: {}", mod_buf.iter().sum::<usize>());
        }
    }

    Ok(())
}
