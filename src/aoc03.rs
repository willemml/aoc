use std::io::BufRead;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::{anychar, char, digit1}, combinator::map_res, error::Error, multi::{fold_many1, many1}, sequence::{delimited, preceded, separated_pair}, IResult
};

fn fnorchar(input: &str) -> IResult<&str, i32, Error<&str>> {
    alt((start_stop,fn_parser,any_as_num))(input)
}

fn any_as_num(input: &str) -> IResult<&str, i32, Error<&str>> {
    let result = anychar(input)?;
    return Ok((result.0, 0));
}

fn fn_parser(input: &str) -> IResult<&str, i32, Error<&str>> {
    let (rem, (a,b)) = preceded(
        tag("mul"),
        delimited(
            char('('),
            separated_pair(
                map_res(digit1, str::parse::<i32>),
                char(','),
                map_res(digit1, str::parse::<i32>),
            ),
            char(')'),
        ),
    )(input)?;
    return Ok((rem, a*b));
}

fn start_stop(input: &str) -> IResult<&str, i32, Error<&str>> {
    let (rem, res) = alt((tag("do()"),tag("don't()")))(input)?;
    Ok(if res == "do()" {
        (rem, -1)
    } else {
        (rem, -2)
    })
}

pub fn p1<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    let result = fold_many1(fnorchar, || 0, |a,f| a+f)(&string).unwrap();
    return Ok(result.1)
}

pub fn p2<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    let (_,arr) = many1(fnorchar)(&string).unwrap();
    let mut total = 0;
    let mut ignoring = false;
    for n in arr {
        if n == -1 {
            ignoring = false;
        } else if n == -2 {
            ignoring = true;
        } else if !ignoring{
            total += n;
        }
    }
    Ok(total)
}
