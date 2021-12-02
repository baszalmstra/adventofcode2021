enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn main() -> anyhow::Result<()> {
    let commands = std::fs::read_to_string("inputs/day2/input")?
        .lines()
        .map(|l| {
            let (op, x) = l.split_once(' ').unwrap();
            let x = x.parse::<isize>().unwrap();
            match op {
                "forward" => Command::Forward(x),
                "down" => Command::Down(x),
                "up" => Command::Up(x),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let end_position = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command {
            Command::Forward(c) => (x + c, y),
            Command::Down(c) => (x, y + c),
            Command::Up(c) => (x, y - c),
        });

    println!("Solution 1: {}", end_position.0 * end_position.1);

    let end_position = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), command| match command {
            Command::Forward(c) => (x + c, y + aim * c, aim),
            Command::Down(c) => (x, y, aim + c),
            Command::Up(c) => (x, y, aim - c),
        });

    println!("Solution 2: {}", end_position.0 * end_position.1);

    Ok(())
}
