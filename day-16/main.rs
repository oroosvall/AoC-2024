// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

#[derive(Clone, Copy)]
enum Dir {
    East,
    South,
    West,
    North,
}

fn get_dir(d: Dir) -> (i32, i32) {
    match d {
        Dir::East => (1, 0),
        Dir::South => (0, 1),
        Dir::West => (-1, 0),
        Dir::North => (0, -1),
    }
}

fn rot_left(d: Dir) -> Dir {
    match d {
        Dir::East => Dir::North,
        Dir::South => Dir::East,
        Dir::West => Dir::South,
        Dir::North => Dir::West,
    }
}

fn rot_right(d: Dir) -> Dir {
    match d {
        Dir::East => Dir::South,
        Dir::South => Dir::West,
        Dir::West => Dir::North,
        Dir::North => Dir::East,
    }
}

fn to_pos(idx: i32, s: i32) -> (i32, i32) {
    (idx % s, idx / s)
}

fn to_idx(p: (i32, i32), s: i32) -> i32 {
    p.1 * s + p.0
}

fn add_pos(p: (i32, i32), d: (i32, i32)) -> (i32, i32) {
    (p.0 + d.0, p.1 + d.1)
}

fn get_best_score(
    map: &String,
    size: (i32, i32),
    start_pos: (i32, i32),
    end_pos: (i32, i32),
) -> i32 {
    let mut score: Vec<i32> = vec![i32::MAX; map.len()];

    let mut to_visit = vec![(Dir::East, start_pos, 0)];
    let mut next = to_visit.pop();

    let s_bytes: &[u8] = { map.as_bytes() };

    while let Some(n) = next {
        let dir = n.0;
        let pos = n.1;
        let s = n.2;

        if s < score[to_idx(pos, size.0) as usize] {
            score[to_idx(pos, size.0) as usize] = s;
            let l_pos = add_pos(pos, get_dir(rot_left(dir)));
            let f_pos = add_pos(pos, get_dir(dir));
            let r_pos = add_pos(pos, get_dir(rot_right(dir)));

            let f_idx = to_idx(f_pos, size.0);
            if s_bytes[f_idx as usize] != b'#' {
                let new_pos = (dir, f_pos, s + 1);
                to_visit.push(new_pos);
            }
            let l_idx = to_idx(l_pos, size.0);
            if s_bytes[l_idx as usize] != b'#' {
                let new_pos = (rot_left(dir), l_pos, s + 1001);
                to_visit.push(new_pos);
            }
            let r_idx = to_idx(r_pos, size.0);
            if s_bytes[r_idx as usize] != b'#' {
                let new_pos = (rot_right(dir), r_pos, s + 1001);
                to_visit.push(new_pos);
            }
        }

        next = to_visit.pop();
    }

    score[to_idx(end_pos, size.0) as usize]
}

fn part_1(v: &Vec<String>) -> i32 {
    let s = (v[0].len() as i32, v.len() as i32);
    let m: String = v
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + &nxt);

    let start_pos = to_pos(m.find('S').unwrap() as i32, s.0);
    let end_pos = to_pos(m.find('E').unwrap() as i32, s.0);
    get_best_score(&m, s, start_pos, end_pos)
}

fn get_best_tiles(
    map: &String,
    size: (i32, i32),
    start_pos: (Dir, i32, i32),
    end_pos: i32,
    v: Vec<i32>,
    mv: i32,
    max_score: i32
) -> Vec<(Vec<i32>, i32)> {
    let mut all_paths = vec![];
    let mut visited = v;
    let mut max_visited = mv;

    let mut to_visit = vec![start_pos];

    let s_bytes: &[u8] = { map.as_bytes() };

    while let Some(n) = to_visit.pop() {
        let dir = n.0;
        let pos_idx = n.1;
        let score = n.2;
        
        if score > max_score {
            break;
        }
        if visited.len() as i32 > max_visited {
            break;
        }
        if visited.contains(&pos_idx) {
            continue;
        }
        visited.push(pos_idx);

        if pos_idx == end_pos {
            all_paths.push((visited, score));
            break;
        }

        let pos = to_pos(pos_idx, size.0);
        {
            let l_idx = to_idx(add_pos(pos, get_dir(rot_left(dir))), size.0);
            let f_idx = to_idx(add_pos(pos, get_dir(dir)), size.0);
            let r_idx = to_idx(add_pos(pos, get_dir(rot_right(dir))), size.0);

            // move forward without any branches
            if s_bytes[f_idx as usize] != b'#'
                && s_bytes[l_idx as usize] == b'#'
                && s_bytes[r_idx as usize] == b'#'
            {
                to_visit.push((dir, f_idx, score + 1));
            }
            // move left without any branches
            else if s_bytes[l_idx as usize] != b'#'
                && s_bytes[f_idx as usize] == b'#'
                && s_bytes[r_idx as usize] == b'#'
            {
                to_visit.push((rot_left(dir), l_idx, score + 1001));
            }
            // move right without any branches
            else if s_bytes[r_idx as usize] != b'#'
                && s_bytes[f_idx as usize] == b'#'
                && s_bytes[l_idx as usize] == b'#'
            {
                to_visit.push((rot_right(dir), r_idx, score + 1001));
            }
            // dead end, break
            else if s_bytes[r_idx as usize] == b'#'
                && s_bytes[f_idx as usize] == b'#'
                && s_bytes[l_idx as usize] == b'#'
            {
                break;
            }
            // we need to branch, 2 or 3 times
            else {
                // branch right and forward
                if s_bytes[r_idx as usize] != b'#'
                    && s_bytes[f_idx as usize] != b'#'
                    && s_bytes[l_idx as usize] == b'#'
                {
                    let b1 = get_best_tiles(&map, size, (rot_right(dir), r_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    for a in b1 {
                        if a.1 == max_score && (a.0.len() as i32) < max_visited {
                            max_visited = a.0.len() as i32;
                        }
                        all_paths.push(a);
                    }
                    let b2 = get_best_tiles(&map, size, (dir, f_idx, score +1), end_pos, visited.clone(), max_visited, max_score);
                    all_paths.extend(b2);

                } else if s_bytes[r_idx as usize] != b'#'
                    && s_bytes[f_idx as usize] == b'#'
                    && s_bytes[l_idx as usize] != b'#'
                {
                    let b1 = get_best_tiles(&map, size, (rot_right(dir), r_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    for a in b1 {
                        if a.1 == max_score && (a.0.len() as i32) < max_visited {
                            max_visited = a.0.len() as i32;
                        }
                        all_paths.push(a);
                    }
                    let b2 = get_best_tiles(&map, size, (rot_left(dir), l_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    all_paths.extend(b2);

                } else if s_bytes[r_idx as usize] == b'#'
                    && s_bytes[f_idx as usize] != b'#'
                    && s_bytes[l_idx as usize] != b'#'
                {
                    let b1 = get_best_tiles(&map, size, (dir, f_idx, score + 1), end_pos, visited.clone(), max_visited, max_score);
                    for a in b1 {
                        if a.1 == max_score && (a.0.len() as i32) < max_visited {
                            max_visited = a.0.len() as i32;
                        }
                        all_paths.push(a);
                    }
                    let b2 = get_best_tiles(&map, size, (rot_left(dir), l_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    all_paths.extend(b2);

                } else if s_bytes[r_idx as usize] != b'#'
                    && s_bytes[f_idx as usize] != b'#'
                    && s_bytes[l_idx as usize] != b'#'
                {
                    let b1 = get_best_tiles(&map, size, (dir, f_idx, score + 1), end_pos, visited.clone(), max_visited, max_score);
                    for a in b1 {
                        if a.1 == max_score && (a.0.len() as i32) < max_visited {
                            max_visited = a.0.len() as i32;
                        }
                        all_paths.push(a);
                    }
                    let b2 = get_best_tiles(&map, size, (rot_left(dir), l_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    for a in b2 {
                        if a.1 == max_score && (a.0.len() as i32) < max_visited {
                            max_visited = a.0.len() as i32;
                        }
                        all_paths.push(a);
                    }
                    let b3 = get_best_tiles(&map, size, (rot_right(dir), r_idx, score + 1001), end_pos, visited.clone(), max_visited, max_score);
                    all_paths.extend(b3);

                }
            }
        }
    }

    all_paths
}

fn part_2(v: &Vec<String>) -> i32 {
    let s = (v[0].len() as i32, v.len() as i32);
    let m: String = v
        .into_iter()
        .fold("".to_string(), |cur: String, nxt: &String| cur + &nxt);

    let start_pos = m.find('S').unwrap() as i32;
    let end_pos = m.find('E').unwrap() as i32;

    let max_visited = i32::MAX;
    let max_score = get_best_score(&m, s, to_pos(start_pos, s.0), to_pos(end_pos, s.0));
    let mut all = get_best_tiles(&m, s, (Dir::East, start_pos, 0), end_pos, vec![], max_visited, max_score);

    // for v in &all {
    //     println!("score: {} len: {}", v.1, v.0.len());
    // }

    all.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let s = all[0].1;
    let all_pos: Vec<Vec<i32>> = all.into_iter().filter(|p| p.1 == s).map(|p| p.0).collect();
    let mut hs = HashSet::new();
    for v in all_pos {
        for pos in v {
            hs.insert(pos);
        }
    }

    hs.len() as i32
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
