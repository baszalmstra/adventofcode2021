use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn idx(self) -> usize {
        (self.1 * 10 + self.0) as usize
    }

    fn neighbours(self) -> impl Iterator<Item = Self> {
        (0..9)
            .map(move |i| {
                let dx = i % 3 - 1;
                let dy = (i - (i % 3)) / 3 - 1;
                Pos(self.0 + dx, self.1 + dy)
            })
            .filter(move |p| (p != &self) && p.0 >= 0 && p.0 < 10 && p.1 >= 0 && p.1 < 10)
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day11/input")?;
    let mut values = input
        .lines()
        .flat_map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .collect_vec();

    let mut total_flash_count = 0;
    let mut step = 0;
    let mut queue = VecDeque::new();
    loop {
        step += 1;

        // Increase energy of all octopuses
        for y in 0..10i32 {
            for x in 0..10i32 {
                let pos = Pos(x, y);
                values[pos.idx()] += 1;
                if values[pos.idx()] > 9 {
                    queue.push_back(pos);
                }
            }
        }

        // Flash each powered-up octopus and increment energy of its neighbours
        while let Some(pos) = queue.pop_front() {
            total_flash_count += 1;
            for neighbour in pos.neighbours() {
                values[neighbour.idx()] += 1;
                if values[neighbour.idx()] == 10 {
                    queue.push_back(neighbour)
                }
            }
        }

        // Reset the energy of all octopuses that have flashed this step
        let mut flash_count_this_step = 0;
        for value in values.iter_mut() {
            if *value >= 10 {
                flash_count_this_step += 1;
                *value = 0;
            }
        }

        // Check end critieria
        if step == 99 {
            println!("Solution 1: {}", total_flash_count);
        } else if flash_count_this_step == 100 {
            println!("Solution 2: {}", step);
            break;
        }
    }

    Ok(())
}
