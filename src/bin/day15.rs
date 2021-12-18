use itertools::Itertools;

struct Grid {
    width: usize,
    height: usize,
    risks: Vec<u8>,
}

impl Grid {
    fn risk_at(&self, (x, y): (usize, usize)) -> usize {
        let wrap_time_x = x / self.width;
        let wrap_time_y = y / self.height;
        let original_risk =
            self.risks[(y as usize % self.height) * self.width + (x as usize % self.width)];

        let mut risk = original_risk as usize + wrap_time_x + wrap_time_y;
        while risk > 9 {
            risk -= 9;
        }

        risk
    }
}

fn parse(input: &str) -> Grid {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().len();
    let risks = lines
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect_vec();
    let height = risks.len() / width;
    Grid {
        width,
        height,
        risks,
    }
}

fn main() -> anyhow::Result<()> {
    let grid = parse(&std::fs::read_to_string("inputs/day15/input")?);

    let (_, total_cost) = pathfinding::directed::astar::astar(
        &(0i32, 0i32),
        |&(x, y)| {
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(|&(x, y)| {
                    x >= 0 && x < grid.width as i32 && y >= 0 && y < grid.height as i32
                })
                .map(|(x, y)| {
                    let pos = (x as usize, y as usize);
                    ((x, y), grid.risk_at(pos))
                })
        },
        |&(x, y)| grid.width - x as usize + grid.height - y as usize,
        |&(x, y)| x as usize == grid.width - 1 && y as usize == grid.height - 1,
    )
    .unwrap();

    println!("Solution 1: {}", total_cost);

    let (_, total_cost) = pathfinding::directed::astar::astar(
        &(0i32, 0i32),
        |&(x, y)| {
            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                .into_iter()
                .map(move |(dx, dy)| (x + dx, y + dy))
                .filter(|&(x, y)| {
                    x >= 0 && x < grid.width as i32 * 5 && y >= 0 && y < grid.height as i32 * 5
                })
                .map(|(x, y)| {
                    let pos = (x as usize, y as usize);
                    ((x, y), grid.risk_at(pos))
                })
        },
        |&(x, y)| grid.width * 5 - x as usize + grid.height * 5 - y as usize,
        |&(x, y)| x as usize == grid.width * 5 - 1 && y as usize == grid.height * 5 - 1,
    )
    .unwrap();

    println!("Solution 2: {}", total_cost);

    Ok(())
}
