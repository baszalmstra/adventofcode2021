fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day3/input")?;
    let mut lines = input.lines().peekable();
    let number_lengths = lines.peek().unwrap().len();
    let numbers = lines
        .map(|num| usize::from_str_radix(num, 2))
        .collect::<Result<Vec<_>, _>>()?;

    let gamma_rate = (0..number_lengths)
        .into_iter()
        .map(|idx| {
            let num_ones = numbers.iter().filter(|&&num| num & (1 << idx) != 0).count();
            if num_ones >= numbers.len() - num_ones {
                1usize << idx
            } else {
                0
            }
        })
        .fold(0, |state, bit_mask| state | bit_mask);

    let epsilon_rate = !gamma_rate & ((1usize << number_lengths) - 1);
    println!("Solution 1: {}", gamma_rate * epsilon_rate);

    let oxygen_generated_rating = find_last_value_by_rating(&numbers, number_lengths, true);
    let cos2_scrubber_rating = find_last_value_by_rating(&numbers, number_lengths, false);

    println!(
        "Solution 2: {}",
        oxygen_generated_rating * cos2_scrubber_rating
    );

    Ok(())
}

fn find_last_value_by_rating(
    values: &[usize],
    number_lengths: usize,
    find_most_common: bool,
) -> usize {
    let mut values = Vec::from(values);
    let mut idx = number_lengths;
    while values.len() > 1 {
        idx -= 1;

        // Get how many ones there are in this bit
        let num_bits_set = values
            .iter()
            .filter(|&&value| (value & (1 << idx)) != 0)
            .count();

        // Get the bit mask to use for this situation
        let mask = (find_most_common && num_bits_set >= values.len() - num_bits_set)
            || (!find_most_common && num_bits_set < values.len() - num_bits_set);

        // Remove items that don't match the bitmask (filter_drain would have been nicer but is
        // unstable)
        let mut i = 0;
        while i < values.len() {
            if (values[i] & (1 << idx) != 0) != mask {
                values.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    values[0]
}
