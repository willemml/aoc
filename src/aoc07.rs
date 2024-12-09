use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    error::Error,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::io::BufRead;

fn parse_line(line: &str) -> IResult<&str, (i128, Vec<i128>), Error<&str>> {
    separated_pair(
        map_res(digit1, str::parse::<i128>),
        char(':'),
        many1(preceded(char(' '), map_res(digit1, str::parse::<i128>))),
    )(line)
}

#[derive(Debug, Clone)]
struct Equation {
    result: i128,
    input: Vec<i128>,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cat,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let mut ops_max = 0u32;
        for i in 0..self.input.len() - 1 {
            ops_max |= 1 << i;
        }
        for flag in 0..=ops_max {
            let mut ans = 0i128;
            for (i, n) in self.input.iter().enumerate() {
                if i == 0 {
                    ans = *n as i128;
                } else if flag & (1 << (i - 1)) == 1 << (i - 1) {
                    ans *= *n as i128;
                } else {
                    ans += *n as i128;
                }
            }
            if ans == self.result {
                return true;
            }
        }
        return false;
    }
    fn is_valid_p2(&self) -> bool {
        let mut ops_max = 0u64;
        for i in 0..((self.input.len() - 1) * 2) {
            ops_max |= 1 << i;
        }

        'outer: for flag in 0..=ops_max {
            let mut ops = Vec::new();
            for i in 0..(self.input.len() - 1) {
                ops.push(match flag >> (i * 2) & 0b11 {
                    0b00 => Op::Add,
                    0b01 => Op::Mul,
                    0b10 => Op::Cat,
                    _ => {
                        continue 'outer;
                    }
                });
            }
            let mut ans = self.input[0];
            for (i, o) in ops.iter().enumerate() {
                match o {
                    Op::Add => {
                        ans += self.input[i + 1];
                    }
                    Op::Mul => {
                        ans *= self.input[i + 1];
                    }
                    Op::Cat => {
                        let n = self.input[i + 1];
                        ans = ans * 10i128.pow(n.ilog10()+1) + n;
                    }
                }
                if ans > self.result {
                    continue 'outer;
                }
            }

            if ans == self.result {
                return true;
            }
        }
        return false;
    }
}

fn read_input<R: BufRead>(mut reader: R) -> Result<Vec<Equation>, std::io::Error> {
    let mut buf = String::new();
    let mut eqs = Vec::new();
    while reader.read_line(&mut buf)? != 0 {
        let (_, (result, input)) = parse_line(&buf).unwrap();
        eqs.push(Equation { result, input });
        buf.clear()
    }
    Ok(eqs)
}

pub fn p1<R: BufRead>(reader: R) -> Result<i128, std::io::Error> {
    let mut total = 0;
    for eq in read_input(reader)? {
        if eq.is_valid() {
            total += eq.result;
        }
    }
    Ok(total)
}

pub fn p2<R: BufRead>(reader: R) -> Result<i128, std::io::Error> {
    let mut total = 0;
    for eq in read_input(reader)? {
        if eq.is_valid_p2() {
            total += eq.result;
        }
    }
    Ok(total)
}
