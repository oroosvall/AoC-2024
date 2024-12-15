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

fn to_pos(idx: i32, s: i32) -> (i32, i32) {
    (idx % s, idx / s)
}

fn to_idx(p: (i32, i32), s: i32) -> i32 {
    p.1 * s + p.0
}

fn get_plant_region(
    m: &mut String,
    s: (usize, usize),
    ch: char,
) -> std::option::Option<(i32, i32)> {
    let mut idx = m.find(ch);

    let mut visited = vec![];
    let mut next_idx = vec![];

    let mut chs = 0;
    let mut fences = 0;

    let s_bytes: &mut [u8] = unsafe { m.as_bytes_mut() };

    while let Some(i) = idx {
        visited.push(i);
        s_bytes[i] = b'#';
        chs += 1;

        let (px, py) = to_pos(i as i32, s.0 as i32);

        if py == 0 || py == s.1 as i32 - 1 {
            fences += 1;
        }
        if px == 0 || px == s.0 as i32 - 1 {
            fences += 1;
        }

        let test_positions = [(px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)];

        for tp in test_positions {
            if tp.1 >= 0 && tp.1 < s.1 as i32 {
                if tp.0 >= 0 && tp.0 < s.0 as i32 {
                    let tp_idx = to_idx(tp, s.0 as i32) as usize;
                    let t_ch = s_bytes[tp_idx] as char;
                    if !visited.contains(&tp_idx) && !next_idx.contains(&tp_idx) && t_ch == ch {
                        next_idx.push(tp_idx);
                    }
                    if t_ch != ch && t_ch != '#' {
                        fences += 1;
                    }
                }
            }
        }

        idx = next_idx.pop();
    }

    for i in visited {
        s_bytes[i] = b' ';
    }

    if chs != 0 {
        return Some((chs, fences));
    }
    None
}

fn part_1(v: &Vec<String>) -> u64 {
    let s = (v[0].len(), v.len());
    let mut m: String = v
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt);

    "ABCDEFGHIJKLMNOPQRSTUVXYZW"
        .chars()
        .map(|plant| {
            let mut sum = 0;
            while let Some(p) = get_plant_region(&mut m, s, plant) {
                sum += p.0 * p.1;
            }
            sum as u64
        })
        .sum::<u64>()
}

type TestPos = ((i32, i32), (i32, i32), (i32, i32));

fn test_corner(m: &mut [u8], s: i32, ch: char, tp: TestPos) -> bool {
    let p1 = to_idx(tp.0, s) as usize;
    let c = to_idx(tp.1, s) as usize;
    let p2 = to_idx(tp.2, s) as usize;

    let cb = ch as u8;

    if m[c] != cb {
        if m[p1] != cb && m[p2] != cb {
            return true;
        } else if m[p1] == cb && m[p2] == cb {
            return true;
        }
    } else {
        if m[p1] != cb && m[p2] != cb {
            return true;
        }
    }
    false
}

fn get_plant_edges(m: &mut String, s: (usize, usize), ch: char) -> std::option::Option<(i32, i32)> {
    let mut idx = m.find(ch);

    let mut visited = vec![];
    let mut next_idx = vec![];

    let mut chs = 0;
    let mut corners = 0;

    let s_bytes: &mut [u8] = unsafe { m.as_bytes_mut() };

    while let Some(i) = idx {
        // println!("{}", i);
        visited.push(i);
        chs += 1;

        let (px, py) = to_pos(i as i32, s.0 as i32);

        if test_corner(
            s_bytes,
            s.0 as i32,
            ch,
            ((px - 1, py), (px - 1, py - 1), (px, py - 1)),
        ) {
            corners += 1;
        }
        if test_corner(
            s_bytes,
            s.0 as i32,
            ch,
            ((px + 1, py), (px + 1, py - 1), (px, py - 1)),
        ) {
            corners += 1;
        }
        if test_corner(
            s_bytes,
            s.0 as i32,
            ch,
            ((px - 1, py), (px - 1, py + 1), (px, py + 1)),
        ) {
            corners += 1;
        }
        if test_corner(
            s_bytes,
            s.0 as i32,
            ch,
            ((px + 1, py), (px + 1, py + 1), (px, py + 1)),
        ) {
            corners += 1;
        }

        let test_positions = [(px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)];

        for tp in test_positions {
            let tp_idx = to_idx(tp, s.0 as i32) as usize;
            let t_ch = s_bytes[tp_idx] as char;
            if !visited.contains(&tp_idx) && !next_idx.contains(&tp_idx) && t_ch == ch {
                next_idx.push(tp_idx);
            }
        }

        idx = next_idx.pop();
    }

    for i in visited {
        s_bytes[i] = b' ';
    }

    if chs != 0 {
        return Some((chs, corners));
    }
    None
}

fn part_2(v: &Vec<String>) -> u64 {
    let mut vv: Vec<String> = v.into_iter().map(|l| " ".to_string() + l + " ").collect();
    vv.push(" ".repeat(vv[0].len()));
    vv.insert(0, " ".repeat(vv[0].len()));

    let s = (vv[0].len(), vv.len());
    let mut m: String = vv
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: String| cur + &nxt);

    "ABCDEFGHIJKLMNOPQRSTUVXYZW"
        .chars()
        .map(|plant| {
            let mut sum = 0;
            while let Some(p) = get_plant_edges(&mut m, s, plant) {
                // println!("{:?}", m);
                // println!("{}: {} * {} -> {}", plant, p.0, p.1, p.0 * p.1);
                sum += p.0 * p.1;
            }
            sum as u64
        })
        .sum::<u64>()
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
