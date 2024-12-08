use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn idx_to_pos(idx: i32, s: i32) -> (i32, i32) {
    let px = idx % s;
    let py = idx / s;
    (px, py)
}

fn pos_to_idx(p: (i32, i32), s: i32) -> i32 {
    (p.1 * s) + p.0
}

fn get_antinodes(nodes: &Vec<i32>, anti: &mut HashSet<i32>, map_size: (i32, i32), pt2: bool) {
    for i in nodes {
        for j in nodes {
            if i != j {
                let p1 = idx_to_pos(*i, map_size.0);
                let p2 = idx_to_pos(*j, map_size.0);

                let d = (p2.0 - p1.0, p2.1 - p1.1);

                let mut t = (p1.0 - d.0, p1.1 - d.1);

                if pt2 {
                    anti.insert(*i);
                }
                loop {
                    if t.0 >= 0 && t.0 < map_size.0 && t.1 >= 0 && t.1 < map_size.1 {
                        let pidx = pos_to_idx(t, map_size.0);
                        anti.insert(pidx);
                    } else {
                        break;
                    }
                    if pt2 {
                        t = (t.0 - d.0, t.1 - d.1);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn part_1(v: &Vec<String>) -> u64 {
    let y_size = v.len() as i32;
    let x_size = v[0].len() as i32;

    let antenna_idx: Vec<(char, usize)> = v
        .iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt)
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch != '.')
        .map(|(idx, ch)| (ch, idx))
        .collect();

    let mut hm: HashMap<char, Vec<i32>> = HashMap::new();
    let mut anti_nodes = HashSet::new();

    for (ch, idx) in antenna_idx {
        if let Some(v) = hm.get_mut(&ch) {
            v.push(idx as i32);
        } else {
            hm.insert(ch, vec![idx as i32]);
        }
    }

    for (_k, p) in &hm {
        get_antinodes(&p, &mut anti_nodes, (x_size, y_size), false);
    }

    anti_nodes.len() as u64
}

fn part_2(v: &Vec<String>) -> u64 {
    let y_size = v.len() as i32;
    let x_size = v[0].len() as i32;

    let antenna_idx: Vec<(char, usize)> = v
        .iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt)
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch != '.')
        .map(|(idx, ch)| (ch, idx))
        .collect();

    let mut hm: HashMap<char, Vec<i32>> = HashMap::new();
    let mut anti_nodes = HashSet::new();

    for (ch, idx) in antenna_idx {
        if let Some(v) = hm.get_mut(&ch) {
            v.push(idx as i32);
        } else {
            hm.insert(ch, vec![idx as i32]);
        }
    }

    for (_k, p) in &hm {
        get_antinodes(&p, &mut anti_nodes, (x_size, y_size), true);
    }

    anti_nodes.len() as u64
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
