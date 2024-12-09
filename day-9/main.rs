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

fn comp(v: &mut Vec<i32>) {
    let mut s = 0;

    for e in (0..v.len()).rev() {
        if v[e] != -1 {
            while v[s] != -1 {
                s += 1;
            }
            if e <= s {
                break;
            }
            // println!("{:?}", v);
            v[s] = v[e];
            v[e] = -1;
        }
    }
    *v = v[..s].to_vec();
}

fn to_ids(v: &Vec<String>) -> Vec<i32> {
    let mut id = 0;
    v[0].chars()
        .enumerate()
        .map(|(idx, c)| {
            let mut vv = vec![];
            for _i in 0..c.to_string().parse::<i32>().unwrap() {
                if idx % 2 == 0 {
                    vv.push(id);
                } else {
                    vv.push(-1);
                }
            }
            if idx % 2 == 0 {
                id += 1;
            }
            return vv;
        })
        .fold(vec![], |mut cur: Vec<i32>, nxt: Vec<i32>| {
            cur.extend(&nxt);
            cur
        })
}

fn part_1(v: &Vec<String>) -> u64 {
    let mut ids = to_ids(v);

    // println!("{:?}", ids);
    comp(&mut ids);
    // println!("{:?}", ids);

    ids.into_iter()
        .enumerate()
        .map(|(idx, v)| v as usize * idx)
        .sum::<usize>() as u64
}

fn get_e(v: &Vec<i32>, e: usize) -> (usize, usize) {
    let mut i = e;
    while i != 0 && v[i] == -1 {
        i -= 1
    }
    let e_val = v[i];
    let mut e_cnt = 0;
    while i - e_cnt != 0 && v[i - e_cnt] == e_val {
        e_cnt += 1
    }

    (i, e_cnt)
}

fn get_s(v: &Vec<i32>, ts: usize) -> usize {
    let mut s = v.len();
    let mut s_len = 0;
    for i in 0..v.len() {
        if v[i] == -1 {
            if s == v.len() {
                s = i;
                s_len += 1;
            } else {
                s_len += 1;
            }
        } else {
            s = v.len();
            s_len = 0;
        }

        if s_len == ts {
            return s;
        }
    }
    return v.len();
}

fn comp_2(v: &mut Vec<i32>) {
    let mut it = v.len() - 1;
    loop {
        let (e, e_cnt) = get_e(v, it);
        let s = get_s(v, e_cnt);

        if e > s {
            if s != v.len() {
                // println!("move {} of {}, to start {}", e_cnt, v[e], s);
                // println!("{:?}", v);
                for i in 0..e_cnt {
                    v[s + i] = v[e - i];
                    v[e - i] = -1;
                }
                // println!("{:?}", v);
            }
        }

        it = e - e_cnt;
        if it == 0 {
            break;
        }
    }
}

fn part_2(v: &Vec<String>) -> u64 {
    let mut ids = to_ids(v);

    // println!("{:?}", ids);
    comp_2(&mut ids);
    // println!("{:?}", ids);

    ids.into_iter()
        .enumerate()
        .map(|(idx, v)| {
            if v != -1 {
                return v as usize * idx;
            }
            0
        })
        .sum::<usize>() as u64
}

fn main() -> Result<(), Error> {
    let mut now = Instant::now();
    let input: Vec<String> = read_to_vec(File::open("input.txt")?)?;
    println!("Read input: {} µs", now.elapsed().as_micros());

    now = Instant::now();
    let r1 = part_1(&input);
    println!("Part 1: {} ms", now.elapsed().as_millis());

    now = Instant::now();
    let r2 = part_2(&input);
    println!("Part 2: {} ms", now.elapsed().as_millis());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}
