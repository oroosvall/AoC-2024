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

#[derive(Debug)]
struct Cpu {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub pc: i64,
    pub buf: String,
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

fn make_cpu(v: Vec<String>) -> Cpu {
    let a = v[0]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let b = v[1]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let c = v[2]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    Cpu {
        a: a,
        b: b,
        c: c,
        pc: 0,
        buf: String::new(),
    }
}

fn new_cpu(a: i64) -> Cpu {
    Cpu {
        a: a,
        b: 0,
        c: 0,
        pc: 0,
        buf: String::new(),
    }
}

fn get_instructions(v: &Vec<String>) -> Vec<i64> {
    let ops = v[0].split(": ").skip(1).next().unwrap();
    ops.split(',')
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn cpu_get_val(cpu: &Cpu, lit: i64) -> i64 {
    match lit {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        7 => 7,
        _ => panic!(),
    }
}

fn cpu_print(cpu: &mut Cpu, val: i64) {
    for ch in val.to_string().chars() {
        cpu.buf += &(ch.to_string() + ",");
    }
}

fn cpu_exec(cpu: &mut Cpu, instr: &Vec<i64>,) {
    while cpu.pc < instr.len() as i64 {
        let op = instr[cpu.pc as usize];
        let lit = instr[(cpu.pc + 1) as usize];
        let val = cpu_get_val(&cpu, lit);
        cpu.pc += 2;

        match op {
            0 => {
                let p = 2_i64.pow(val as u32);
                cpu.a /= p;
            }
            1 => {
                cpu.b = cpu.b ^ lit;
            }
            2 => {
                cpu.b = val.rem_euclid(8);
            }
            3 => {
                if cpu.a != 0 {
                    cpu.pc = lit;
                }
            }
            4 => {
                cpu.b = cpu.b ^ cpu.c;
            }
            5 => {
                cpu_print(cpu, val.rem_euclid(8));
            }
            6 => {
                let p = 2_i64.pow(val as u32);
                cpu.b = cpu.a / p;
            }
            7 => {
                let p = 2_i64.pow(val as u32);
                cpu.c = cpu.a / p;
            }
            _ => panic!(),
        }
    }
}

fn part_1(v: &Vec<String>) -> String {
    let (cpu_strs, instructions) = split_at_empty(&v);
    let mut cpu = make_cpu(cpu_strs);
    let opcodes = get_instructions(&instructions);

    cpu_exec(&mut cpu, &opcodes);

    cpu.buf.pop();
    cpu.buf
}

fn part_2(v: &Vec<String>) -> i64 {
    let (cpu_strs, instructions) = split_at_empty(&v);
    let mut cpu = make_cpu(cpu_strs);
    let opcodes = get_instructions(&instructions);

    let output = "";
    for ch in &opcodes {
        cpu.buf += &(ch.to_string() + ",");
    }

    let xor_val = 0b1111;
    let trunc = 0b111111111111111111111111111111111111111111111111;
    let mut a = 0b111111111111111111111111111111111111111111111111;
    // let mut a = 0b0;

    // let mut a = 0b111100011001011110100110101011111111111111111111;
    // let mut a = 0b111100011001011110101100010100100101110110100100;
    // let mut a = 0b111100011001011010011001010100000000000000010110;

    // while cpu.buf != instructions[0]
    // for _ in 0..1024

    let mut shift = 0;
    // loop
    {
        cpu = new_cpu(a);
        cpu_exec(&mut cpu, &opcodes);

        // cpu.buf.pop();
        // println!("{:#048b} -> {} {:?}", a, cpu.buf.len(), cpu.buf);
        // a = cpu.buf.replace(",", "").parse::<i64>().unwrap();
        // a = a << 1;
        if cpu.buf == output {
            // break;
        }
        a = (a ^ (xor_val << shift)) & trunc;
        shift += 1;
    }

    0
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
