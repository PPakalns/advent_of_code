#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

struct Pos {
    x: usize,
    y: usize,
}

fn cnt_slope(input: &[Vec<u8>], dx: usize, dy: usize) -> usize {
    let h = input.len();
    let w = input[0].len();

    let mut pos = Pos { x: 0, y: 0 };
    let mut cnt = 0;
    while pos.y < h {
        if input[pos.y][pos.x] == b'#' {
            cnt += 1;
        }
        pos.x += dx;
        while pos.x >= w {
            pos.x -= w;
        }
        pos.y += dy;
    }
    cnt
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    cnt_slope(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    cnt_slope(input, 1, 1)
        * cnt_slope(input, 3, 1)
        * cnt_slope(input, 5, 1)
        * cnt_slope(input, 7, 1)
        * cnt_slope(input, 1, 2)
}
