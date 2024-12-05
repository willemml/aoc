use std::io::BufRead;

fn checkxmas(chars: &[char]) -> bool {
    chars == &['X', 'M', 'A', 'S'] || chars == &['S', 'A', 'M', 'X']
}

pub fn p1<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    let chars: Vec<char> = string
        .chars()
        .map(|c| match c {
            'X' | 'M' | 'A' | 'S' | '\n' => c,
            _ => ' ',
        })
        .collect();
    let mut count = 0;
    let mut split: Vec<&[char]> = chars.split(|c| c == &'\n').collect();
    let last = split.pop().unwrap();
    if !last.is_empty() {
        split.push(last);
    }
    for (_i, r) in split.iter().enumerate() {
        for c in 0..r.len() - 3 {
            if checkxmas(&r[c..c + 4]) {
                count += 1;
            }
        }
    }
    for c in 0..split[0].len() {
        for r in 0..split.len() - 3 {
            let split = &split;
            let chars = [
                split[r][c],
                split[r + 1][c],
                split[r + 2][c],
                split[r + 3][c],
            ];
            if checkxmas(&chars) {
                count += 1;
            }
        }
    }
    for c in 0..split[0].len() - 3 {
        for r in 0..split.len() - 3 {
            let chars1 = [
                split[r][c],
                split[r + 1][c + 1],
                split[r + 2][c + 2],
                split[r + 3][c + 3],
            ];
            let chars2 = [
                split[r][c + 3],
                split[r + 1][c + 2],
                split[r + 2][c + 1],
                split[r + 3][c],
            ];
            if checkxmas(&chars1) {
                count += 1;
            }
            if checkxmas(&chars2) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn check_a(arr: &[&[char]], r: usize, c: usize) -> bool {
    let top = [
        arr[r - 1][c - 1],
        arr[r - 1][c + 1],
        arr[r + 1][c - 1],
        arr[r + 1][c + 1],
    ];
    match top {
        ['M', 'S', 'M', 'S']
        | ['M', 'M', 'S', 'S']
        | ['S', 'S', 'M', 'M']
        | ['S', 'M', 'S', 'M'] => {
            // println!("{} {}", top[0], top[1]);
            // println!(" A ");
            // println!("{} {}", top[2], top[3]);
            // println!();
            true
        }
        _ => false,
    }
}

pub fn p2<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    let chars: Vec<char> = string.chars().collect();
    let mut count = 0;
    let mut split: Vec<&[char]> = chars.split(|c| c == &'\n').collect();
    let last = split.pop().unwrap();
    if !last.is_empty() {
        split.push(last);
    }
    for r in 1..split.len() - 1 {
        for c in 1..split[r].len() - 1 {
            if split[r][c] == 'A' {
                if check_a(&split, r, c) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}
