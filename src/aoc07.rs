use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    error::Error,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::io::BufRead;

fn parse_line(line: &str) -> IResult<&str, (i64, Vec<i32>), Error<&str>> {
    separated_pair(
        map_res(digit1, str::parse::<i64>),
        char(':'),
        many1(preceded(char(' '), map_res(digit1, str::parse::<i32>))),
    )(line)
}

#[derive(Debug)]
struct Equation {
    result: i64,
    input: Vec<i32>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let mut ops_max = 0u32;
        for i in 0..self.input.len() - 1 {
            ops_max |= 1 << i;
        }
        for flag in 0..=ops_max {
            let mut ans = 0i64;
            for (i, n) in self.input.iter().enumerate() {
                if i == 0 {
                    ans = *n as i64;
                } else if flag & (1 << (i - 1)) == 1 << (i - 1) {
                    ans *= *n as i64;
                } else {
                    ans += *n as i64;
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

// 3312272362250 is too high
pub fn p1<R: BufRead>(reader: R) -> Result<i64, std::io::Error> {
    let mut total = 0;
    for eq in read_input(reader)? {
        if eq.is_valid() {
            total += eq.result;
        }
    }
    Ok(total)
}

pub fn p2<R: BufRead>(_reader: R) -> Result<i32, std::io::Error> {
    Ok(0)
}
