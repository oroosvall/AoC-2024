use std::collections::HashMap;
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

fn do_moves(d: (i32, Dir), x_size: i32, y_size: i32, g: &mut str) -> i32 {
    let s_bytes: &mut [u8] = unsafe { g.as_bytes_mut() };

    let mut px = d.0 % x_size;
    let mut py = d.0 / x_size;
    let mut dir = d.1;

    loop {
        s_bytes[to_idx(px, py, x_size) as usize] = 'X' as u8;

        match dir {
            Dir::Up => {
                py -= 1;
                if py == -1 {
                    break;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    py += 1; // restore
                    dir = Dir::Right;
                }
            }
            Dir::Right => {
                px += 1;
                if px == x_size {
                    break;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    px -= 1; // restore
                    dir = Dir::Down;
                }
            }
            Dir::Down => {
                py += 1;
                if py == y_size {
                    break;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    py -= 1; // restore
                    dir = Dir::Left;
                }
            }
            Dir::Left => {
                px -= 1;
                if px == -1 {
                    break;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    px += 1; // restore
                    dir = Dir::Up;
                }
            }
        };
    }
    g.chars().filter(|c| *c == 'X' || *c == 'O').count() as i32
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
    let s_bytes: &mut [u8] = unsafe { g.as_bytes_mut() };

    let mut px = d.0 % x_size;
    let mut py = d.0 / x_size;
    let mut dir = d.1;

    let mut f_obj = (-1, -1, dir.clone());
    let mut nr_loops = 0;

    let mut moves: HashMap<i32, Dir> = HashMap::new();

    loop {
        let mut reset = false;

        let pidx = to_idx(px, py, x_size);
        if let Some(d) = moves.get(&pidx) {
            if *d == dir {
                // println!("{}, {} -> n turns: {}", py, px, nturn - f_obj.3);
                nr_loops += 1;
                reset = true;
            }
        } else {
            moves.insert(pidx, dir.clone());
        }

        match dir {
            Dir::Up => {
                py -= 1;
                if py == -1 {
                    if f_obj.0 != -1 {
                        reset = true;
                    } else {
                        break;
                    }
                } else if f_obj.0 == -1 && s_bytes[to_idx(px, py, x_size) as usize] != '#' as u8 {
                    f_obj = (px, py, dir.clone());
                    py += 1; // restore
                    dir = Dir::Right;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    py += 1; // restore
                    dir = Dir::Right;
                }
            }
            Dir::Right => {
                px += 1;
                if px == x_size {
                    if f_obj.0 != -1 {
                        reset = true;
                    } else {
                        break;
                    }
                } else if f_obj.0 == -1 && s_bytes[to_idx(px, py, x_size) as usize] != '#' as u8 {
                    f_obj = (px, py, dir.clone());
                    px -= 1; // restore
                    dir = Dir::Down;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    px -= 1; // restore
                    dir = Dir::Down;
                }
            }
            Dir::Down => {
                py += 1;
                if py == y_size {
                    if f_obj.0 != -1 {
                        reset = true;
                    } else {
                        break;
                    }
                } else if f_obj.0 == -1 && s_bytes[to_idx(px, py, x_size) as usize] != '#' as u8 {
                    f_obj = (px, py, dir.clone());
                    py -= 1; // restore
                    dir = Dir::Left;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    py -= 1; // restore
                    dir = Dir::Left;
                }
            }
            Dir::Left => {
                px -= 1;
                if px == -1 {
                    if f_obj.0 != -1 {
                        reset = true;
                    } else {
                        break;
                    }
                } else if f_obj.0 == -1 && s_bytes[to_idx(px, py, x_size) as usize] != '#' as u8 {
                    f_obj = (px, py, dir.clone());
                    px += 1; // restore
                    dir = Dir::Up;
                } else if s_bytes[to_idx(px, py, x_size) as usize] == '#' as u8 {
                    px += 1; // restore
                    dir = Dir::Up;
                }
            }
        };

        if reset {
            px = f_obj.0;
            py = f_obj.1;
            dir = f_obj.2.clone();
            f_obj = (-1, -1, dir.clone());
            moves = HashMap::new();
        }
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
    println!("Part 2: {} µs", now.elapsed().as_micros());

    println!("Result 1: {}\nResult 2: {}", r1, r2);

    Ok(())
}
