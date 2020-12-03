pub struct Password {
    low: usize,
    high: usize,
    pwd: String,
    symb: u8,
}

fn parse_line(line: &str) -> Password {
    let data: Vec<&str> = line.split(' ').collect();
    let range: Vec<&str> = data[0].split('-').collect();
    let symb = data[1].as_bytes()[0];
    let pwd = data[2];

    Password {
        low: range[0].parse().expect("integer"),
        high: range[1].parse().expect("integer"),
        pwd: pwd.into(),
        symb,
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input.lines().map(parse_line).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|pvalue| {
            let symbol_count = bytecount::count(pvalue.pwd.as_bytes(), pvalue.symb);
            (pvalue.low <= symbol_count) && (symbol_count <= pvalue.high)
        })
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|pvalue| {
            let bytes = pvalue.pwd.as_bytes();
            [pvalue.low, pvalue.high]
                .iter()
                .map(|&idx| bytes.get(idx - 1).unwrap_or(&0))
                .filter(|&&sym| sym == pvalue.symb)
                .count()
                == 1
        })
        .count()
}
