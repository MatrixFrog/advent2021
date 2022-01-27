fn part1() -> i32 {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    let values = input();
    for index in 0..12 {
        let mut zeros = 0;
        let mut ones = 0;
        for v in &values {
            match &v.chars().nth(index).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                nonbit => panic!("Not a bit: {}", nonbit),
            }
        }
        if zeros > ones {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

fn more_common_bit(values: &[&str], index: usize) -> char {
    let mut zeros = 0;
    let mut ones = 0;
    for v in values {
        match v.chars().nth(index).unwrap() {
            '0' => zeros += 1,
            '1' => ones += 1,
            nonbit => panic!("Not a bit: {}", nonbit),
        }
    }
    if zeros > ones {
        '0'
    } else {
        '1'
    }
}

fn calc_part2_oxygen() -> String {
    let mut values = input();
    for index in 0..12 {
        let mcb = more_common_bit(&values, index);
        values.retain(|v| v.chars().nth(index).unwrap() == mcb);
        if values.len() == 1 {
            return values[0].to_string();
        }
    }
    panic!("This shouldn't happen???");
}

fn calc_part2_co2() -> String {
    let mut values = input();
    for index in 0..12 {
        let mcb = more_common_bit(&values, index);
        values.retain(|v| v.chars().nth(index).unwrap() != mcb);
        if values.len() == 1 {
            return values[0].to_string();
        }
    }
    panic!("This shouldn't happen???");
}

fn part2() -> i32 {
    let oxygen = calc_part2_oxygen();
    let co2 = calc_part2_co2();
    i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&co2, 2).unwrap()
}

fn input() -> Vec<&'static str> {
    include_str!("input.txt").lines().collect()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}
