use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::time::Instant;

fn read_to_vec<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let lines = br.lines().map(|l| l.expect("Parse error")).collect();

    Ok(lines)
}

fn transpose(v: &Vec<String>) -> Vec<String>
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner.chars().nth(i).unwrap()).collect::<String>())
        .collect()
}

fn test_diag(v: &Vec<String>, sx: usize, sy: usize, t: &str) -> bool {

    assert!(!v.is_empty());
    let val = (0..t.len())
        .map(|i| v[sy + i].chars().nth(sx + i).unwrap())
        .collect::<String>();

    val == t
}

fn test_diag2(v: &Vec<String>, sx: usize, sy: usize, t: &str) -> bool {

    assert!(!v.is_empty());
    let val = (0..t.len())
        .map(|i| v[sy - i].chars().nth(sx + i).unwrap())
        .collect::<String>();

    val == t
}

fn check_diag(v: &Vec<String>, t: &str) -> i32 {

    let mut n = 0;
    assert!(!v.is_empty());
    for x in 0..v[0].len() - (t.len()-1) {
        for y in 0..(v.len()) {
            if y < v.len()-(t.len()-1) {
                if test_diag(v, x, y, t) {
                    n += 1;
                }
            }
        }
    }

    for x in 0..v[0].len() - (t.len()-1) {
        for y in t.len()-1..(v.len()) {
            if test_diag2(v, x, y, t) {
                n += 1;
            }
        }
    }

    n
}

fn part_1(vec: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for l in vec {
        sum += l.match_indices("XMAS").count() as i32;
        sum += l.match_indices("SAMX").count() as i32;
    }

    let vv = transpose(vec);
    for l in &vv {
        sum += l.match_indices("XMAS").count() as i32;
        sum += l.match_indices("SAMX").count() as i32;
    }

    sum += check_diag(vec, "XMAS");
    sum += check_diag(vec, "SAMX");

    // sum += check_diag(&vv, "XMAS");
    // sum += check_diag(&vv, "SAMX");

    sum
}

fn test_mas(v: &Vec<String>, t: ((char,char),char,(char,char))) -> i32 {
    let mut sum = 0;
    for x in 0..v.len()-2 {
        for y in 0..v.len()-2 {
            let c1 = v[y].chars().nth(x).unwrap() == t.0.0;
            let c3 = v[y].chars().nth(x+2).unwrap()== t.0.1;
            let c5 = v[y+1].chars().nth(x+1).unwrap()== t.1;
            let c7 = v[y+2].chars().nth(x).unwrap()== t.2.0;
            let c9 = v[y+2].chars().nth(x+2).unwrap()== t.2.1;

            if c1 && c3 && c5 && c7 && c9 {
                sum += 1;
            }
        }
    }
    sum
}

fn part_2(v: &Vec<String>) -> i32 {

    let mas1 = (
        ('M', 'M'), 'A', ('S', 'S')
    );
    let mas2 = (
        ('S','S'), 'A', ('M', 'M')
    );
    let mas3 = (
        ('S', 'M'), 'A', ('S', 'M')
    );
    let mas4 = (
        ('M', 'S'),'A',('M', 'S')
    );

    test_mas(v, mas1) + test_mas(v, mas2) + test_mas(v, mas3) + test_mas(v, mas4)
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
