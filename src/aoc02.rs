use std::io::BufRead;

fn fails(numbers: Vec<i32>) -> bool {
    let mut fail = false;
    let mut prev = None;
    let mut incr = None;
    for n in numbers.clone() {
        if prev.is_none() {
            prev = Some(n);
            continue;
        }
        {
            let prev = prev.unwrap();
            if n == prev {
                fail = true;
                break;
            }
            if incr.is_none() {
                incr = Some(n > prev);
            }
            if incr.unwrap() {
                if n < prev {
                    fail = true;
                    break;
                } else if (n - prev) > 3 {
                    fail = true;
                    break;
                }
            } else {
                if n > prev {
                    fail = true;
                    break;
                } else if (prev - n) > 3 {
                    fail = true;
                    break;
                }
            }
        }
        prev = Some(n);
    }
    return fail;
}

pub fn p1<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut buf = String::new();
    let mut count = 0;
    while reader.read_line(&mut buf)? != 0 {
        let split = buf.split(' ');
        let line: Vec<&str> = split.collect();
        let numbers: Vec<i32> = line
            .into_iter()
            .filter_map(|i| {
                let t = i.trim();
                if t.len() < 1 {
                    None
                } else {
                    Some(t.parse().unwrap())
                }
            })
            .collect();
        if !fails(numbers.clone()) {
            count += 1;
        }
        buf.clear();
    }
    Ok(count)
}


pub fn p2<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut buf = String::new();
    let mut count = 0;
    while reader.read_line(&mut buf)? != 0 {
        let split = buf.split(' ');
        let line: Vec<&str> = split.collect();
        let numbers: Vec<i32> = line
            .into_iter()
            .filter_map(|i| {
                let t = i.trim();
                if t.len() < 1 {
                    None
                } else {
                    Some(t.parse().unwrap())
                }
            })
            .collect();
        if !fails(numbers.clone()) {
            count += 1;
        } else {
            for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                if !fails(numbers) {
                    count += 1;
                    break;
                }
            }
        }
        buf.clear();
    }
    Ok(count)
}

