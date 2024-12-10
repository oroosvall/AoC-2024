// use std::collections::HashMap;
// use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn to_pos(idx: i32, s: i32) -> (i32, i32) {
    (idx % s, idx / s)
}

fn to_idx(p: (i32, i32), s: i32) -> i32 {
    p.1 * s + p.0
}

fn is_in_range(p: (i32, i32), s: (i32, i32)) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < s.0 && p.1 < s.1
}

fn get_nr_trailheads(
    m: &Vec<i32>,
    h: i32,
    idx: i32,
    s: (i32, i32),
    mut v: &mut Vec<i32>,
    pt2: bool,
) -> i32 {
    // standing at a 9, reached trailhead
    if !pt2 {
        if v.contains(&idx) {
            return 0;
        }
        v.push(idx);
    }
    if m[idx as usize] == 9 {
        return 1;
    } else {
        let p = to_pos(idx, s.0);
        return [
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ]
        .iter()
        .filter(|np| is_in_range(**np, s))
        .filter(|np| m[to_idx(**np, s.0) as usize] == h + 1)
        .map(|np| {
            // println!("{}, {}", idx, h + 1);
            get_nr_trailheads(m, h + 1, to_idx(*np, s.0), s, &mut v, pt2)
        })
        .sum::<i32>();
    }
}

fn part_1(v: &Vec<String>) -> i32 {
    let y_size = v.len() as i32;
    let x_size = v[0].len() as i32;
    let m: Vec<i32> = v
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt)
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap_or(-1))
        .collect();

    m.iter()
        .enumerate()
        .filter(|(_, h)| **h == 0)
        .map(|(idx, h)| {
            // println!("{}, {}", idx, h);
            let mut visited = vec![];
            get_nr_trailheads(&m, *h, idx as i32, (x_size, y_size), &mut visited, false)
        })
        .sum()
}

fn part_2(v: &Vec<String>) -> i32 {
    let y_size = v.len() as i32;
    let x_size = v[0].len() as i32;
    let m: Vec<i32> = v
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt)
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap_or(-1))
        .collect();

    m.iter()
        .enumerate()
        .filter(|(_, h)| **h == 0)
        .map(|(idx, h)| {
            // println!("{}, {}", idx, h);
            let mut visited = vec![];
            get_nr_trailheads(&m, *h, idx as i32, (x_size, y_size), &mut visited, true)
        })
        .sum()
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
