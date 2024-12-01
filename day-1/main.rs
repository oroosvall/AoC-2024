
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::{Instant};

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines()
        .map(|l| l.expect("Parse error"))
        .collect();

    Ok(lines)
}

fn part_1(vec :&Vec<String>) -> i32
{
    let mut num_1 : Vec<i32> = vec!();
    let mut num_2 : Vec<i32> = vec!();
    for l in vec {
        let mut parts = l.split("   ");
        let (n1,n2) = (parts.next().expect("string").parse::<i32>().unwrap(), parts.next().expect("string").parse::<i32>().unwrap());
        num_1.push(n1);
        num_2.push(n2);
    }
    num_1.sort();
    num_2.sort();

    let it = num_1.iter().zip(num_2.iter());
    it.enumerate().map(|(_,(a,b))| (a-b).abs()).sum()
}

fn part_2(vec :&Vec<String>) -> i32
{
    let mut num_1 : Vec<i32> = vec!();
    let mut num_2 : Vec<i32> = vec!();
    for l in vec {
        let mut parts = l.split("   ");
        let (n1,n2) = (parts.next().expect("string").parse::<i32>().unwrap(), parts.next().expect("string").parse::<i32>().unwrap());
        num_1.push(n1);
        num_2.push(n2);
    }

    num_1.into_iter().map(|n| num_2.clone().into_iter().filter(|n2| *n2 == n).collect::<Vec<i32>>().len() as i32 * n).sum()
}

fn main() -> Result<(), Error>
{
    let mut now = Instant::now();
    let input : Vec<String> = read_to_vec(File::open("input.txt")?)?;
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