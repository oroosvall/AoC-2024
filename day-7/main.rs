use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn try_maths(expected: u64, a: &str, rem: Vec<&str>, pt2: bool) -> bool {
    let v1 = a.parse::<u64>().unwrap();
    let v2 = rem[0].parse::<u64>().unwrap();
    if rem.len() == 1 {
        let st = a.to_string() + rem[0];
        if expected == v1 * v2 {
            return true;
        } else if expected == v1 + v2 {
            return true;
        } else if expected == st.parse::<u64>().unwrap() {
            return true;
        }
    } else {
        let st = (v1 * v2).to_string();
        let mut r = try_maths(expected, &st, rem[1..].to_vec(), pt2);
        if !r {
            let st = (v1 + v2).to_string();
            r = try_maths(expected, &st, rem[1..].to_vec(), pt2);
        }
        if !r && pt2 {
            let st = a.to_string() + rem[0];
            r = try_maths(expected, &st, rem[1..].to_vec(), pt2);
        }
        return r;
    }

    false
}

fn part_1(v: &Vec<String>) -> u64 {
    let vec: Vec<(u64, Vec<&str>)> = v
        .into_iter()
        .map(|l| {
            let mut s = l.split(": ");
            (s.next().unwrap(), s.next().unwrap())
        })
        .map(|(a, b)| {
            (
                a.parse::<u64>().unwrap(),
                b.split(' ').into_iter().map(|x| x).collect::<Vec<&str>>(),
            )
        })
        .filter(|(res, vals)| try_maths(*res, vals[0], vals[1..].to_vec(), false))
        .collect();
    vec.into_iter().map(|(a, _)| a).sum::<u64>() as u64
}

fn part_2(v: &Vec<String>) -> u64 {
    let vec: Vec<(u64, Vec<&str>)> = v
        .into_iter()
        .map(|l| {
            let mut s = l.split(": ");
            (s.next().unwrap(), s.next().unwrap())
        })
        .map(|(a, b)| {
            (
                a.parse::<u64>().unwrap(),
                b.split(' ').into_iter().map(|x| x).collect::<Vec<&str>>(),
            )
        })
        .filter(|(res, vals)| try_maths(*res, vals[0], vals[1..].to_vec(), true))
        .collect();
    vec.into_iter().map(|(a, _)| a).sum::<u64>() as u64
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
