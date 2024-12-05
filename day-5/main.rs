use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn split_stuff(v: &Vec<String>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut v2: Vec<Vec<i32>> = Vec::new();

    let mut first_part = true;
    for l in v {
        if first_part {
            if l.len() == 0 {
                first_part = false;
                continue;
            }

            let mut nrs = l.split('|');

            let key = nrs.next().unwrap().parse::<i32>().unwrap();
            let value = nrs.next().unwrap().parse::<i32>().unwrap();

            if let Some(vec) = m.get_mut(&key) {
                vec.push(value);
            } else {
                m.insert(key, vec![value]);
            }
        } else {
            let vec = l
                .split(',')
                .into_iter()
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            v2.push(vec);
        }
    }

    (m, v2)
}

fn get_value(pages: &Vec<i32>, m: &HashMap<i32, Vec<i32>>) -> i32 {
    for i in 0..pages.len() - 1 {
        let val = pages[i];
        if let Some(map) = &m.get(&val) {
            for j in (i + 1)..pages.len() {
                let cont = map.contains(&pages[j]);
                if !cont {
                    return 0;
                }
            }
        } else {
            return 0;
        }
    }
    pages[pages.len() / 2]
}

fn part_1(v: &Vec<String>) -> i32 {
    let (m, pages_col) = split_stuff(v);

    pages_col.into_iter().map(|p| get_value(&p, &m)).sum()
}

fn fix_pages(pages: &Vec<i32>, m: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut np: Vec<i32> = vec![];

    for pidx in 0..pages.len() {
        let p = pages[pidx];
        // println!("np: {:?}", np);
        for i in 0..np.len() + 1 {
            let mut test = np.clone();
            test.insert(i, p);
            // println!("testing: {:?}", test);
            if get_value(&test, &m) != 0 {
                np = test;
            }
        }
    }

    get_value(&np, &m)
}

fn part_2(v: &Vec<String>) -> i32 {
    let (m, pages_col) = split_stuff(v);

    pages_col
        .into_iter()
        .map(|p| {
            if get_value(&p, &m) == 0 {
                return fix_pages(&p, &m);
            }
            0
        })
        .sum()
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
