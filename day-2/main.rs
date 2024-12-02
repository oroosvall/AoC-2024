use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

#[derive(PartialEq, Eq)]
enum State {
    Initial,
    Dec,
    Inc,
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut s = State::Initial;
    for i in 0..nums.len() - 1 {
        let d = nums[i] - nums[i + 1];
        if d == 1 || d == 2 || d == 3 {
            if s == State::Initial {
                s = State::Inc;
            } else if s != State::Inc {
                return false
            }
        } else if d == -1 || d == -2 || d == -3 {
            if s == State::Initial {
                s = State::Dec;
            } else if s != State::Dec {
                return false
            }
        } else {
            return false
        }
    }
    true
}

fn part_1(vec: &Vec<String>) -> i32 {
    let mut safe_nums: i32 = 0;
    for l in vec {
        let parts = l.split(" ");
        let mut nums: Vec<i32> = vec![];
        for p in parts {
            let n = p.parse::<i32>().unwrap();
            nums.push(n);
        }

        if is_safe(&nums) {
            safe_nums += 1;
        }
    }

    safe_nums
}

fn part_2(vec: &Vec<String>) -> i32 {
    let mut safe_nums: i32 = 0;
    for l in vec {
        let parts = l.split(" ");
        let mut nums: Vec<i32> = vec![];
        for p in parts {
            let n = p.parse::<i32>().unwrap();
            nums.push(n);
        }

        if is_safe(&nums) {
            safe_nums += 1;
        } else {
            for i in 0..nums.len() {
                let mut nn = nums.clone();
                nn.remove(i);
                if is_safe(&nn) {
                    safe_nums += 1;
                    break;
                }
            }
        }
    }

    safe_nums
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
