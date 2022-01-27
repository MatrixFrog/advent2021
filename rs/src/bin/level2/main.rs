fn input() -> impl Iterator<Item = &'static str> {
    include_str!("input.txt").lines()
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn update(pos: &mut Position, command: &str) {
    let mut parts = command.split(' ');
    let command_type = parts.next().unwrap();
    let value: i32 = parts.next().unwrap().parse().unwrap();

    match command_type {
        "down" => pos.aim += value,
        "up" => pos.aim -= value,
        "forward" => {
            pos.horizontal += value;
            pos.depth += pos.aim * value;
        }
        _ => panic!("Unexpected command type {}", command_type),
    }
}

fn main() {
    let mut pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for line in input() {
        update(&mut pos, line)
    }

    println!("{}", pos.depth * pos.horizontal);
}
