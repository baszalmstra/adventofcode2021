use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pos(isize, isize);

impl Pos {
    fn neighbours(self) -> impl Iterator<Item = Pos> {
        [
            Pos(self.0, self.1 - 1),
            Pos(self.0 - 1, self.1),
            Pos(self.0, self.1 + 1),
            Pos(self.0 + 1, self.1),
        ]
        .into_iter()
    }
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day9/input")?;
    let width = input.lines().next().unwrap().len();
    let grid = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let height = grid.len() / width;

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let value = grid[y * width + x];
            let max_neighbours = Pos(x as isize, y as isize)
                .neighbours()
                .filter(|p| p.0 >= 0 && p.0 < width as isize && p.1 >= 0 && p.1 < height as isize)
                .map(|Pos(x, y)| grid[y as usize * width + x as usize])
                .min()
                .unwrap();
            if max_neighbours > value {
                sum += value + 1;
            }
        }
    }

    println!("Solution 1: {}", sum);

    let mut basins = grid
        .iter()
        .map(|v| {
            if *v == 9 {
                Cell::Border
            } else {
                Cell::Basin(1)
            }
        })
        .collect::<Vec<_>>();

    fn find(basins: &mut [Cell], idx: usize) -> Option<usize> {
        let basin_idx = match basins[idx] {
            Cell::Basin(_) => idx,
            Cell::PartOfBasin(basin) => {
                let basin_idx = find(basins, basin).unwrap();
                basins[idx] = Cell::PartOfBasin(basin_idx);
                basin_idx
            }
            Cell::Border => return None,
        };
        Some(basin_idx)
    }

    fn union(basins: &mut [Cell], a: usize, b: usize) {
        let basin_a = find(basins, a);
        let basin_b = find(basins, b);
        if basin_a == basin_b {
            return;
        }
        if let (Some(a), Some(b)) = (basin_a, basin_b) {
            match (basins[a], basins[b]) {
                (Cell::Basin(a_count), Cell::Basin(b_count)) => {
                    basins[b] = Cell::PartOfBasin(a);
                    basins[a] = Cell::Basin(a_count + b_count);
                }
                _ => unreachable!(),
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            if let Cell::Border = basins[i] {
                continue;
            } else {
                if x > 0 {
                    union(&mut basins, i, i - 1);
                }
                if y + 1 < height {
                    union(&mut basins, i, i + width);
                }
                if x + 1 < width {
                    union(&mut basins, i, i + 1);
                }
                if y > 0 {
                    union(&mut basins, i, i - width);
                }
            }
        }
    }

    let mut basin_count = basins
        .into_iter()
        .filter_map(|c| {
            if let Cell::Basin(count) = c {
                Some(count)
            } else {
                None
            }
        })
        .collect_vec();
    basin_count.sort_unstable();
    let total: usize = basin_count.into_iter().rev().take(3).product();

    println!("Solution 2: {}", total);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Basin(usize),
    PartOfBasin(usize),
    Border,
}
