#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.parse().expect("number format"))
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    for a in input.iter() {
        for b in input.iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("Solution not found!");
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    for a in input.iter() {
        for b in input.iter() {
            for c in input.iter() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("Solution not found!");
}
