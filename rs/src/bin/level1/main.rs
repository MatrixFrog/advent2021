fn input() -> impl Iterator<Item = i32> {
    include_str!("input.txt")
        .lines()
        .map(|l| l.parse().unwrap())
}

fn part1(values: &[i32]) -> i32 {
    let mut total: i32 = 0;

    let mut previous: Option<i32> = None;
    for current in values {
        if let Some(value) = previous {
            if *current > value {
                total += 1;
            }
        }
        previous = Some(*current);
    }

    total
}

fn part2(values: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for (i, v) in values.iter().enumerate() {
        if i < 3 {
            continue;
        }
        let back3 = values.get(i - 3);
        if let Some(previous_value) = back3 {
            if v > previous_value {
                total += 1;
            }
        }
    }

    total
}

fn main() {
    let values = input().collect::<Vec<i32>>();
    println!("{}", part1(&values));
    println!("{}", part2(&values));
}
