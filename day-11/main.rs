use std::collections::HashMap;
// use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

// Rules
// 0 -> 1
// len % 2 == 0 -> split half
// else mul 2024
fn blink(b: &Vec<(String, u64)>) -> Vec<(String, u64)> {
    
    let mut hm: HashMap<String, u64>= HashMap::new();

    for (s, c) in b {
        if s == "0" {
            if let Some(v) = hm.get_mut("1") {
                *v += c;
            } else {
                hm.insert("1".to_string(), *c);
            }
        } else if s.len() % 2 == 0 {
            let (s1, s2) = s.split_at(s.len() / 2);
            {
                let nr = s1.parse::<u64>().unwrap().to_string();
                if let Some(v) = hm.get_mut(&nr) {
                    *v += c;
                } else {
                    hm.insert(nr, *c);
                }
            }
            {
                let nr = s2.parse::<u64>().unwrap().to_string();
                if let Some(v) = hm.get_mut(&nr) {
                    *v += c;
                } else {
                    hm.insert(nr, *c);
                }
            }
        } else {
            let nr = (s.parse::<u64>().unwrap() * 2024).to_string();
            if let Some(v) = hm.get_mut(&nr) {
                *v += c;
            } else {
                hm.insert(nr, *c);
            }
        }
    }

    hm.into_iter().collect()
}

fn part_1(v: &Vec<String>) -> u64 {

    let mut bucket : Vec<(String, u64)> = v[0].split(' ').into_iter().map(|v| (v.to_string(), 1)).collect();

    for _i in 0..25 {
        bucket = blink(&bucket);
    }
    bucket.into_iter().map(|(_,v)| v).sum()
}

fn part_2(v: &Vec<String>) -> u64 {
    let mut bucket : Vec<(String, u64)> = v[0].split(' ').into_iter().map(|v| (v.to_string(), 1)).collect();

    for _i in 0..75 {
        bucket = blink(&bucket);
    }
    bucket.into_iter().map(|(_,v)| v).sum()
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
