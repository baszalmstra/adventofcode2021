use std::fmt::{Display, Formatter};

struct Dots {
    width: usize,
    height: usize,
    dots: Vec<bool>,
}

impl Dots {
    pub fn fold(&self, fold: &Fold) -> Self {
        match fold {
            Fold::Horizontal(line_y) => {
                let mut result = Dots {
                    width: self.width,
                    height: *line_y,
                    dots: vec![false; self.width * line_y],
                };

                for y in 0..*line_y {
                    for x in 0..self.width {
                        let top = self.dots[y * self.width + x];
                        let bottom_line = line_y + (line_y - y);
                        let bottom = if bottom_line < self.height {
                            self.dots[bottom_line * self.width + x]
                        } else {
                            false
                        };
                        result.dots[y * self.width + x] = top || bottom;
                    }
                }

                result
            }
            Fold::Vertical(line_x) => {
                let mut result = Dots {
                    width: *line_x,
                    height: self.height,
                    dots: vec![false; line_x * self.height],
                };

                for y in 0..self.height {
                    for x in 0..*line_x {
                        let left = self.dots[y * self.width + x];
                        let right_line = line_x + (line_x - x);
                        let right = if right_line < self.width {
                            self.dots[y * self.width + right_line]
                        } else {
                            false
                        };
                        result.dots[y * line_x + x] = left || right;
                    }
                }

                result
            }
        }
    }
}

impl Display for Dots {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.dots[y * self.width + x] {
                    write!(f, "#")?
                } else {
                    write!(f, " ")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn parse(input: &str) -> anyhow::Result<(Dots, Vec<Fold>)> {
    let mut lines = input.lines();
    let mut dots = Vec::<(usize, usize)>::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let (x, y) = line
            .split_once(',')
            .ok_or_else(|| anyhow::anyhow!("not a coordinate"))?;
        dots.push((x.parse()?, y.parse()?));
    }

    let width = dots
        .iter()
        .map(|(x, _)| *x)
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty"))?
        + 1;
    let height = dots
        .iter()
        .map(|(_, y)| *y)
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty"))?
        + 1;

    let mut result = Dots {
        width,
        height,
        dots: vec![false; width * height],
    };
    for (x, y) in dots {
        result.dots[y * result.width + x] = true
    }

    let mut folds = Vec::new();
    for line in lines.by_ref() {
        if let Some(horizontal) = line.strip_prefix("fold along y=") {
            folds.push(Fold::Horizontal(horizontal.parse()?));
        } else if let Some(vertical) = line.strip_prefix("fold along x=") {
            folds.push(Fold::Vertical(vertical.parse()?));
        } else {
            unreachable!()
        }
    }

    Ok((result, folds))
}

fn main() -> anyhow::Result<()> {
    let (mut dots, folds) = parse(&std::fs::read_to_string("inputs/day13/input")?)?;

    for (i, fold) in folds.into_iter().enumerate() {
        dots = dots.fold(&fold);
        if i == 0 {
            let dot_count = dots.dots.iter().filter(|dot| **dot).count();
            println!("Solution 1: {}", dot_count);
        }
    }

    println!("Solution 2:\n{}", dots);

    Ok(())
}
