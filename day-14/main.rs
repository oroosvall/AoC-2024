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

fn to_vals(s: &str) -> (i64, i64) {
    let mut vv = s.split(",");
    let x = vv.next().unwrap().parse::<i64>().unwrap();
    let y = vv.next().unwrap().parse::<i64>().unwrap();
    (x, y)
}

fn to_drones(v: &Vec<String>) -> Vec<((i64, i64), (i64, i64))> {
    v.into_iter()
        .map(|l| {
            let mut s1 = l.split(' ');
            let p = s1.next().unwrap().split('=').skip(1).next().unwrap();
            let v = s1.next().unwrap().split('=').skip(1).next().unwrap();
            (to_vals(&p), to_vals(&v))
        })
        .collect()
}

fn part_1(v: &Vec<String>) -> i64 {
    let s = (101, 103);
    let t = 100;
    let drones: Vec<(i64, i64)> = to_drones(&v)
        .into_iter()
        .map(|d| {
            let p = d.0;
            let v = d.1;
            (p.0 + v.0 * t, p.1 + v.1 * t)
        })
        .map(|p| (p.0.rem_euclid(s.0), p.1.rem_euclid(s.1)))
        .collect();

    let q1 = drones
        .iter()
        .filter(|(x, y)| *x < s.0 / 2 && *y < s.1 / 2)
        .count();
    let q2 = drones
        .iter()
        .filter(|(x, y)| *x > s.0 / 2 && *y < s.1 / 2)
        .count();
    let q3 = drones
        .iter()
        .filter(|(x, y)| *x < s.0 / 2 && *y > s.1 / 2)
        .count();
    let q4 = drones
        .iter()
        .filter(|(x, y)| *x > s.0 / 2 && *y > s.1 / 2)
        .count();

    // println!("drones: {:?}", (q1,q2,q3,q4));
    (q1 * q2 * q3 * q4) as i64
}

fn part_2(v: &Vec<String>) -> i64 {
        let s = (101, 103);
        let t = 103; // loop max, should be enough to find pattern
    
        let mut drones = to_drones(&v);

        let middle = (s.0/2, s.1/2);
        let mut max = (0,0);
        let mut max_idx = (0,0);

        for i in 1..t {
            drones = drones
                .into_iter()
                .map(|d| {
                    let p = d.0;
                    let v = d.1;
                    (((p.0 + v.0).rem_euclid(s.0), (p.1 + v.1).rem_euclid(s.1)), v)
                })
                .collect();

            let x = drones.iter().filter(|(p,_)| (p.0 - middle.0).abs() < 10).count();
            let y = drones.iter().filter(|(p,_)| (p.1 - middle.1).abs() < 20).count();

            if x > max.0 {
                max.0 = x;
                max_idx.0 = i;
            }

            if y > max.1 {
                max.1 = y;
                max_idx.1 = i;
            }
        }

        let xps:Vec<i64> = (0..t).map(|i| max_idx.0 + 101 * i).collect();
        for i in 0..t {
            let y_val = max_idx.1 + 103 * i;
            if xps.contains(&y_val) {
                return y_val;
            }
        }
        0
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
