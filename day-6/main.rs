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

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn to_idx(x: i32, y: i32, s: i32) -> i32 {
    (y * s) + x
}

fn get_dir(d: Dir) -> (i32, i32) {
    match d {
        Dir::Up => (0, -1),
        Dir::Right => (1, 0),
        Dir::Down => (0, 1),
        Dir::Left => (-1, 0),
    }
}

fn next_dir(d: Dir) -> Dir {
    match d {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn do_moves(d: (i32, Dir), x_size: i32, y_size: i32, g: &mut str) -> i32 {
    let s_bytes: &mut [u8] = unsafe { g.as_bytes_mut() };

    let mut px = d.0 % x_size;
    let mut py = d.0 / x_size;
    let mut dir = d.1;

    loop {
        s_bytes[to_idx(px, py, x_size) as usize] = 'X' as u8;

        let mv = get_dir(dir.clone());

        px += mv.0;
        py += mv.1;

        if py == -1 || px == -1 || px == x_size || py == y_size {
            break;
        } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
            px -= mv.0;
            py -= mv.1;
            dir = next_dir(dir.clone());
        }
    }
    g.chars().filter(|c| *c == 'X').count() as i32
}

fn part_1(v: &Vec<String>) -> i32 {
    let y = v.len() as i32;
    let x = v[0].len() as i32;

    let mut g = v
        .iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt);

    let idx = g.find('^').unwrap() as i32;

    do_moves((idx, Dir::Up), x, y, &mut g)
}

fn find_loops(d: (i32, Dir), x_size: i32, y_size: i32, g: &mut str) -> i32 {

    let binding = String::from(&mut *g);
    let path = binding.match_indices("X");

    let s_bytes: &mut [u8] = unsafe { g.as_bytes_mut() };

    let mut nr_loops = 0;

    for (f_pos, _) in path {

        if f_pos == d.0 as usize {
            continue;
        }

        let mut px = d.0 % x_size;
        let mut py = d.0 / x_size;
        let mut dir = d.1.clone();

        s_bytes[f_pos] = '#' as u8;
        let mut moves: HashMap<i32, Dir> = HashMap::new();

        loop {

            let pidx = to_idx(px, py, x_size);
            if let Some(d) = moves.get(&pidx) {
                if *d == dir {
                    nr_loops += 1;
                    break;
                }
            } else {
                moves.insert(pidx, dir.clone());
            }

            let mv = get_dir(dir.clone());

            px += mv.0;
            py += mv.1;

            if py == -1 || px == -1 || px == x_size || py == y_size {
                break;
            } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                px -= mv.0;
                py -= mv.1;
                dir = next_dir(dir.clone());
            }
        }
        s_bytes[f_pos] = '.' as u8;
    }
    nr_loops
}

fn part_2(v: &Vec<String>) -> i32 {
    let y = v.len() as i32;
    let x = v[0].len() as i32;

    let mut g = v
        .iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + nxt);

    let idx = g.find('^').unwrap() as i32;

    do_moves((idx, Dir::Up), x, y, &mut g);
    find_loops((idx, Dir::Up), x, y, &mut g)
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
    println!("Part 2: {} ms", now.elapsed().as_millis());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}
