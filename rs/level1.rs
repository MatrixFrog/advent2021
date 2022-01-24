use std::fs;

fn part1() -> i32 {
    let contents = fs::read_to_string("input1.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut total: i32 = 0;

    let mut previous: Option<i32> = None;
    for line in lines {
        let current: i32 = line.parse().unwrap();
        match previous {
            Some(value) => {
                if current > value {
                    total += 1;
                }
            }
            None => {}
        }
        previous = Some(current);
    }

    total
}

fn part2() -> i32 {
    let contents = fs::read_to_string("input1.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();
    let values: Vec<i32> = lines.map(|l| l.parse::<i32>().unwrap()).collect();

    let mut total: i32 = 0;

    for (i, v) in values.iter().enumerate() {
        if i < 3 {
            continue;
        }
        let back3 = values.get(i - 3);
        back3.map(|previous_value| {
            if v > previous_value {
                total += 1;
            }
        });
    }

    total
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
