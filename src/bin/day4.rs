use itertools::Itertools;

#[derive(Debug, Clone)]
struct Board {
    values: Vec<usize>,
    marked: [bool; 25],
}

impl Board {
    pub fn new(values: Vec<usize>) -> Self {
        Self {
            values,
            marked: [false; 25],
        }
    }

    pub fn get_unmarked_values(&self) -> impl Iterator<Item = usize> + '_ {
        self.marked.iter().enumerate().filter_map(|(idx, marked)| {
            if !*marked {
                Some(self.values[idx])
            } else {
                None
            }
        })
    }

    pub fn mark(&mut self, value: usize) {
        for idx in 0..25 {
            if self.values[idx] == value {
                self.marked[idx] = true
            }
        }
    }

    pub fn bingo(&self) -> bool {
        for row in 0..5 {
            if self
                .marked
                .iter()
                .skip(5 * row)
                .take(5)
                .all(|marked| *marked)
                || self
                    .marked
                    .iter()
                    .skip(row)
                    .step_by(5)
                    .take(5)
                    .all(|marked| *marked)
            {
                return true;
            }
        }
        false
    }
}

fn parse(input: &str) -> anyhow::Result<(Vec<usize>, Vec<Board>)> {
    let mut lines = input.lines().peekable();
    let order = lines
        .next()
        .ok_or(anyhow::anyhow!("empty input"))?
        .split(',')
        .map(|num| num.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut boards = Vec::new();
    let chunks = lines.chunks(6);
    for chunk in &chunks {
        let values = chunk
            .flat_map(|line| line.split(' '))
            .filter(|num| !num.is_empty())
            .map(|num| num.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        boards.push(Board::new(values));
    }
    Ok((order, boards))
}

fn main() -> anyhow::Result<()> {
    let (order, mut boards) = parse(std::fs::read_to_string("inputs/day4/input")?.as_str())?;

    let mut last_winning_board_score = None;
    for value in order {
        for board in boards.iter_mut() {
            board.mark(value);
        }
        while let Some(idx) = (0..boards.len()).find(|idx| boards[*idx].bingo()) {
            let board = boards.swap_remove(idx);
            let unmarked_sum: usize = board.get_unmarked_values().sum();
            let score = value * unmarked_sum;

            if last_winning_board_score.is_none() {
                println!("Solution 1: {}", score);
            }

            last_winning_board_score = Some(score);
        }
    }

    println!("Solution 2: {}", last_winning_board_score.unwrap());

    Ok(())
}
