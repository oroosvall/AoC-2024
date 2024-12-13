// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn do_split(s: &String, ch: char) -> (i64, i64) {
    let mut vv = s.split(": ").skip(1).next().unwrap().split(", ");
    let x = vv
        .next()
        .unwrap()
        .split(ch)
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let y = vv
        .next()
        .unwrap()
        .split(ch)
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    (x, y)
}

fn to_machines(v: &Vec<String>) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    v.chunks(4)
        .map(|vs| {
            (
                do_split(&vs[0], '+'),
                do_split(&vs[1], '+'),
                do_split(&vs[2], '='),
            )
        })
        .collect()
}

fn get_presses3(m: &((i64, i64), (i64, i64), (i64, i64))) -> i64 {

    let ap = m.0;
    let bp = m.1;
    let p = m.2;

    let det = ap.0 * bp.1 - ap.1 * bp.0;
    if det == 0 {
        return 0;
    }

    let mut a = bp.1 * p.0 - bp.0 * p.1;
    let mut b = ap.0 * p.1 - ap.1 * p.0;

    if a % det != 0 || b % det != 0 {
        return 0;
    }

    a /= det;
    b /= det;

    3 * a + b
}

fn part_1(v: &Vec<String>) -> i64 {
    to_machines(v)
        .into_iter()
        .map(|m| get_presses3(&m))
        .sum::<i64>()
}

fn add_price_pos(
    m: &((i64, i64), (i64, i64), (i64, i64)),
    s: i64,
) -> ((i64, i64), (i64, i64), (i64, i64)) {
    (m.0, m.1, (m.2 .0 + s, m.2 .1 + s))
}

fn part_2(v: &Vec<String>) -> i64 {
    to_machines(v)
        .into_iter()
        .map(|m| add_price_pos(&m, 10000000000000))
        .map(|m| get_presses3(&m))
        .sum::<i64>()
}

fn main() -> Result<(), Error> {
    let mut now = Instant::now();
    let input: Vec<String> = read_to_vec(File::open("input.txt")?)?;
    println!("Read input: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r1 = part_1(&input);
    println!("Part 1: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r2 = part_2(&input);
    println!("Part 2: {} µs", now.elapsed().as_micros());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}
