fn calc_part1(input: &str) {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    let values: Vec<&str> = input.lines().collect();
    for index in 0..12 {
        let mut zeros = 0;
        let mut ones = 0;
        for v in &values {
            match &v[index..index + 1] {
                "0" => zeros += 1,
                "1" => ones += 1,
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

    println!("gamma={}, epsilon={}", gamma, epsilon);

    println!(
        "{}",
        isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
    );
}

fn more_common_bit(values: &Vec<&str>, index: usize) -> String {
    let mut zeros = 0;
    let mut ones = 0;
    for v in values {
        match &v[index..index + 1] {
            "0" => zeros += 1,
            "1" => ones += 1,
            nonbit => panic!("Not a bit: {}", nonbit),
        }
    }
    if zeros > ones {
        return "0".to_string();
    } else {
        return "1".to_string();
    }
}

fn calc_part2_oxygen(input: &str) -> &str {
    let mut values: Vec<&str> = input.lines().collect();
    for index in 0..12 {
        let mcb = more_common_bit(&values, index);
        values.retain(|v| v[index..index + 1] == mcb);
        if values.len() == 1 {
            return values[0];
        }
    }
    panic!("This shouldn't happen???");
}

fn calc_part2_co2(input: &str) -> &str {
    let mut values: Vec<&str> = input.lines().collect();
    for index in 0..12 {
        let mcb = more_common_bit(&values, index);
        values.retain(|v| v[index..index + 1] != mcb);
        if values.len() == 1 {
            return values[0];
        }
    }
    panic!("This shouldn't happen???");
}

fn calc_part2(input: &str) -> (i32, i32) {
    let oxygen = calc_part2_oxygen(input);
    let co2 = calc_part2_co2(input);
    return (
        i32::from_str_radix(&oxygen, 2).unwrap(),
        i32::from_str_radix(&co2, 2).unwrap(),
    );
}

fn main() {
    // calc_part1(include_str!("./input3.txt"));
    let (o, c) = calc_part2(include_str!("./input3.txt"));
    println!("{}", o * c);
}
