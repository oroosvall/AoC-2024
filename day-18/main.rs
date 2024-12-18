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

// fn to_pos(idx: i32, s: i32) -> (i32, i32) {
//     (idx % s, idx / s)
// }

fn to_idx(p: (i32, i32), s: i32) -> i32 {
    p.1 * s + p.0
}

fn add_pos(p: (i32, i32), d: (i32, i32)) -> (i32, i32) {
    (p.0 + d.0, p.1 + d.1)
}

fn get_path(map: &String, size: (i32, i32), start_pos: (i32, i32), end_pos: (i32, i32)) -> i32 {
    let mut score: Vec<i32> = vec![i32::MAX; map.len()];

    let mut to_visit = vec![(start_pos, 0)];

    let s_bytes: &[u8] = { map.as_bytes() };

    while let Some(n) = to_visit.pop() {
        let pos = n.0;
        let s = n.1;

        if s < score[to_idx(pos, size.0) as usize] {
            score[to_idx(pos, size.0) as usize] = s;
            let move_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for d in move_dirs {
                let p = add_pos(pos, d);
                let idx = to_idx(p, size.0);
                if s_bytes[idx as usize] != b'#' {
                    to_visit.push((p, s + 1));
                }
            }
        }
    }

    score[to_idx(end_pos, size.0) as usize]
}

fn do_grid(v: &Vec<(i32, i32)>, gs: usize, iter: usize, end_pos: (i32, i32)) -> i32 {
    let size = (gs, gs);
    let mut map = "#".repeat((size.0 + 1) * (size.1 + 1));

    let s_bytes: &mut [u8] = unsafe { map.as_bytes_mut() };

    for y in 1..(size.1 - 1) {
        for x in 1..(size.0 - 1) {
            s_bytes[to_idx((x as i32, y as i32), size.0 as i32) as usize] = b'.';
        }
    }

    for i in 0..iter {
        let (x, y) = v[i];
        s_bytes[to_idx((x as i32, y as i32), size.0 as i32) as usize] = b'#';
    }

    let start_pos = (1, 1);

    get_path(&map, (size.0 as i32, size.1 as i32), start_pos, end_pos)
}

fn part_1(v: &Vec<String>) -> i32 {
    let corruptions = v
        .into_iter()
        .map(|l| {
            let mut p = l.split(',');
            let x = p.next().unwrap().parse::<i32>().unwrap();
            let y = p.next().unwrap().parse::<i32>().unwrap();
            (x + 1, y + 1)
        })
        .collect();
    do_grid(&corruptions, 73, 1024, (71, 71))
}

fn divide(min: usize, max: usize) -> usize {
    (min + max) / 2
}

fn part_2(v: &Vec<String>) -> String {
    let corruptions: Vec<(i32, i32)> = v
        .into_iter()
        .map(|l| {
            let mut p = l.split(',');
            let x = p.next().unwrap().parse::<i32>().unwrap();
            let y = p.next().unwrap().parse::<i32>().unwrap();
            (x + 1, y + 1)
        })
        .collect();

    let mut range = (0, corruptions.len());
    while range.0 + 1 != range.1 {
        let mid = divide(range.0, range.1);
        let completed = do_grid(&corruptions, 73, mid, (71, 71));
        if completed == i32::MAX {
            range.1 = mid;
        } else {
            range.0 = mid;
        }
    }

    format!(
        "{},{}",
        corruptions[range.1 - 1].0 - 1,
        corruptions[range.1 - 1].1 - 1
    )
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
