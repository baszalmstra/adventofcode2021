use std::ops::RangeInclusive;

struct Bounds {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl Bounds {
    pub fn in_bounds(&self, pos: (isize, isize)) -> bool {
        self.x.contains(&pos.0) && self.y.contains(&pos.1)
    }
}

fn velocity_in_bounds(mut velocity: (isize, isize), bounds: &Bounds) -> (bool, isize) {
    let mut highest_y = 0;
    let mut x = 0;
    let mut y = 0;
    loop {
        if bounds.in_bounds((x, y)) {
            return (true, highest_y);
        }

        if x > *bounds.x.end() || y < *bounds.y.start() {
            return (false, highest_y);
        }

        x += velocity.0;
        y += velocity.1;

        highest_y = highest_y.max(y);

        velocity.0 -= velocity.0.signum();
        velocity.1 -= 1;
    }
}

fn main() -> anyhow::Result<()> {
    let bounds = Bounds {
        x: 94..=151,
        y: -156..=-103,
    };

    let min_x_velocity =
        (0.5 * (8.0 * *bounds.x.start() as f64 + 1.0).sqrt() - 0.5).ceil() as isize;
    let min_y_velocity = *bounds.y.start();
    let max_x_velocity = *bounds.x.end();

    let mut highest_pos = -1;
    let mut in_bounds_count = 0;

    for x in min_x_velocity..=max_x_velocity {
        for y in min_y_velocity..=200 {
            let (in_bounds, highest_y) = velocity_in_bounds((x, y), &bounds);
            if in_bounds {
                if highest_pos < highest_y {
                    highest_pos = highest_y;
                }
                // println!("In bounds: {},{}", x,y);
                in_bounds_count += 1;
            }
        }
    }

    println!("Solution 1: {}", highest_pos);
    println!("Solution 2: {}", in_bounds_count);

    Ok(())
}
