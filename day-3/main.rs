use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

#[derive(Debug, PartialEq)]
enum TestResult {
    ValidChar,
    ValidNum,
    ValidNumFChar,
    Invalid,
}

fn test_is_valid(ch: char, find: &str, idx: usize) -> TestResult {
    let fc = find.chars().nth(idx).unwrap();

    // println!("testing {} == {}", ch, fc);

    if fc == '?' {
        if ch.is_digit(10) {
            return TestResult::ValidNum;
        } else if ch == find.chars().nth(idx + 1).unwrap() {
            return TestResult::ValidNumFChar;
        }
        return TestResult::Invalid;
    } else if ch == fc {
        return TestResult::ValidChar;
    }
    TestResult::Invalid
}

fn parse_mul(text: &str, find: &str) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = Vec::new();

    let mut t_idx = 0;
    let mut idx = 0;
    let mut max_digits = 3;
    let mut wrk = "".to_string();
    let mut p = (-1, -1);

    while t_idx < text.len() {
        let c = text.chars().nth(t_idx).unwrap();

        let res = test_is_valid(c, find, idx);

        // println!("{} {:?}", c, res);

        if res == TestResult::Invalid && idx != 0 {
            wrk = "".to_string();
            idx = 0;
            max_digits = 3;
            p = (-1, -1);
            continue;
        }

        if res == TestResult::ValidNum {
            if max_digits != 0 {
                wrk.push(c);
                max_digits -= 1;
            } else {
                wrk = "".to_string();
                idx = 0;
                max_digits = 3;
                p = (-1, -1);
                continue;
            }
        } else if res == TestResult::ValidNumFChar {
            if max_digits == 3 {
                wrk = "".to_string();
                idx = 0;
                max_digits = 3;
                p = (-1, -1);
                continue;
            } else if p.0 == -1 {
                p.0 = wrk.parse::<i32>().unwrap();
                wrk = "".to_string();
                max_digits = 3;
                idx += 2
            } else {
                p.1 = wrk.parse::<i32>().unwrap();
                idx += 2
            }

            if p.0 != -1 && p.1 != -1 {
                v.push(p);
                wrk = "".to_string();
                idx = 0;
                max_digits = 3;
                p = (-1, -1);
            }
        } else if res == TestResult::ValidChar {
            idx += 1;
        }

        t_idx += 1;
    }

    v
}

fn do_dnt(txt: &str) -> String {
    let dstr = "do()";
    let dntstr = "don't()";

    let mut from = 0;
    let mut to = txt.find(dntstr).unwrap();
    let mut st = String::new();

    loop {
        let f = from;
        let t = to;

        // println!("{} -> {}", f, t);

        st += &txt[f..t];

        // find the next do
        let mut off = txt[t + dntstr.len()..].find(dstr);
        if off != None {
            let o = off.unwrap();
            from = t + dntstr.len() + o + dstr.len()
        } else {
            from = usize::MAX;
            break;
        }

        off = txt[from + dstr.len()..].find(dntstr);
        if off != None {
            let o = off.unwrap();
            to = from + dstr.len() + o + dntstr.len();
        } else  {
            to = usize::MAX;
            break;
        }
    }

    if from != usize::MAX && to == usize::MAX {
        // println!("{} -> end", f);

        st += &txt[from..];
    }

    // println!("{}", st);

    st
}

fn part_1(vec: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    let m = r"mul(?,?)";
    let mut ll = "".to_string();
    for l in vec {
        ll += l;
    }

    for (a, b) in parse_mul(&ll, m) {
        // println!("{},{}", a, b);
        sum += a * b;
    }

    sum
}

fn part_2(vec: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    let m = r"mul(?,?)";
    let mut ll = "".to_string();
    for l in vec {
        ll += l;
    }

    let l2 = do_dnt(&ll);
    for (a, b) in parse_mul(&l2, m) {
        // println!("{},{}", a, b);
        sum += a * b;
    }

    sum
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
