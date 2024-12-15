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

fn split_at_empty(v: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut v1 = vec![];
    let mut v2 = vec![];

    let mut first = true;
    for l in v {
        if l.len() == 0 {
            first = false;
            continue;
        }
        if first {
            v1.push(l.clone());
        } else {
            v2.push(l.clone());
        }
    }
    (v1, v2)
}

fn to_new_pos(pos: (i32, i32), dir: char) -> (i32, i32) {
    match dir {
        '<' => (pos.0 - 1, pos.1),
        'v' => (pos.0, pos.1 + 1),
        '>' => (pos.0 + 1, pos.1),
        '^' => (pos.0, pos.1 - 1),
        _ => panic!("not allowed"),
    }
}

fn do_move(map: &mut [u8], pos: (i32, i32), s: i32, dir: char, ch: char) {
    let where_to = to_new_pos(pos, dir);
    let idx = to_idx(where_to, s) as usize;
    if map[idx] == b'#' {
        return;
    } else if map[idx] == b'O' {
        do_move(map, where_to, s, dir, 'O');
    } else if map[idx] == b'[' || map[idx] == b']' {
        if dir == '^' || dir == 'v' {
            let ch = map[idx] as char;
            if ch == '[' {
                do_move(map, where_to, s, dir, '[');
                do_move(map, (where_to.0 + 1, where_to.1), s, dir, ']');
            } else {
                do_move(map, (where_to.0 - 1, where_to.1), s, dir, '[');
                do_move(map, where_to, s, dir, ']');
            }
        } else {
            do_move(map, where_to, s, dir, map[idx] as char);
        }
    }
    map[idx] = ch as u8;
    map[to_idx(pos, s) as usize] = b'.';
}

fn try_move(map: &mut [u8], pos: (i32, i32), s: i32, dir: char) -> (i32, i32) {
    let where_to = to_new_pos(pos, dir);
    let idx = to_idx(where_to, s) as usize;
    if map[idx] == b'#' {
        return pos;
    } else if map[idx] == b'O' {
        let new_box_pos = try_move(map, where_to, s, dir);
        if new_box_pos.0 == where_to.0 && new_box_pos.1 == where_to.1 {
            return pos;
        } else {
            return where_to;
        }
    } else if map[idx] == b'[' || map[idx] == b']' {
        // need to do special case when moving
        if dir == '^' || dir == 'v' {
            let box_p1_new_pos = try_move(map, where_to, s, dir);

            if box_p1_new_pos.0 == where_to.0 && box_p1_new_pos.1 == where_to.1 {
                return pos; // failed to move one half of the box
            }

            let where_to_p2;
            let box_p2_new_pos;
            if map[idx] == b'[' {
                where_to_p2 = (where_to.0 + 1, where_to.1);
                box_p2_new_pos = try_move(map, where_to_p2, s, dir);
            } else {
                where_to_p2 = (where_to.0 - 1, where_to.1);
                box_p2_new_pos = try_move(map, where_to_p2, s, dir);
            }

            if box_p2_new_pos.0 == where_to_p2.0 && box_p2_new_pos.1 == where_to_p2.1 {
                return pos; // failed to move one half of the box
            }
        } else {
            let new_box_pos = try_move(map, where_to, s, dir);
            if new_box_pos.0 == where_to.0 && new_box_pos.1 == where_to.1 {
                return pos;
            } else {
                return where_to;
            }
        }
    }
    where_to
}

fn part_1(v: &Vec<String>) -> i32 {
    let (map, instructions) = split_at_empty(&v);

    let s = (map[0].len() as i32, map.len() as i32);
    let mut m: String = map
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: String| cur + &nxt);

    let instr: String = instructions
        .into_iter()
        .fold("".to_string(), |cur, nxt| cur + &nxt);

    let mut robot_pos = to_pos(m.find('@').unwrap() as i32, s.0);
    {
        let s_bytes: &mut [u8] = unsafe { m.as_bytes_mut() };

        for i in instr.chars() {
            let new_robot_pos = try_move(s_bytes, robot_pos, s.0, i);
            if !(new_robot_pos.0 == robot_pos.0 && new_robot_pos.1 == robot_pos.1) {
                do_move(s_bytes, robot_pos, s.0, i, '@');
            }
            robot_pos = new_robot_pos;
            // for y in 0..s.1 {
            //     for x in 0..s.0 {
            //         print!("{}", s_bytes[to_idx((x, y), s.0) as usize] as char);
            //     }
            //     println!();
            // }
            // println!();
        }
    }

    m.match_indices('O')
        .map(|(i, _)| to_pos(i as i32, s.0 as i32))
        .map(|p| p.1 * 100 + p.0)
        .sum::<i32>()
}

fn make_larger(s: &String) -> String {
    let mut ss = String::new();
    for ch in s.chars() {
        if ch == '#' {
            ss += "##";
        } else if ch == 'O' {
            ss += "[]";
        } else if ch == '.' {
            ss += "..";
        } else if ch == '@' {
            ss += "@."
        }
    }
    ss
}

fn part_2(v: &Vec<String>) -> i32 {
    let (map, instructions) = split_at_empty(&v);

    let s = (map[0].len() as i32 * 2, map.len() as i32);
    let mut m: String = map
        .into_iter()
        .map(|l| make_larger(&l))
        .fold("".to_string(), |cur: String, nxt: String| cur + &nxt);

    let instr: String = instructions
        .into_iter()
        .fold("".to_string(), |cur, nxt| cur + &nxt);

    let mut robot_pos = to_pos(m.find('@').unwrap() as i32, s.0);
    {
        let s_bytes: &mut [u8] = unsafe { m.as_bytes_mut() };

        for i in instr.chars() {
            let new_robot_pos = try_move(s_bytes, robot_pos, s.0, i);
            if !(new_robot_pos.0 == robot_pos.0 && new_robot_pos.1 == robot_pos.1) {
                do_move(s_bytes, robot_pos, s.0, i, '@');
            }
            robot_pos = new_robot_pos;
        }
        // for y in 0..s.1 {
        //     for x in 0..s.0 {
        //         print!("{}", s_bytes[to_idx((x, y), s.0) as usize] as char);
        //     }
        //     println!();
        // }
        // println!();
    }

    m.match_indices('[')
        .map(|(i, _)| to_pos(i as i32, s.0 as i32))
        .map(|p| p.1 * 100 + p.0)
        .sum::<i32>()
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
