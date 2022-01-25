fn raw_input() -> &'static str {
    include_str!("input.txt")
}

fn input() -> Vec<i32> {
    let mut distances = Vec::new();
    for l in raw_input().split(',') {
        distances.push(l.trim().parse().unwrap());
    }
    distances
}

fn solve() -> i32 {
    let distances = input();
    let max = *distances.iter().max().unwrap();
    let min = *distances.iter().min().unwrap();
    (min..max)
        .map(|pos| distances.iter().map(|d| (d - pos).abs()).sum())
        .min().unwrap()
}

fn main() {
    println!("{}",solve())
}
