use std::{collections::HashSet, io::BufRead};

fn read_map<R: BufRead>(
    mut reader: R,
) -> Result<(Pos, (usize, usize), Vec<Vec<bool>>), std::io::Error> {
    let mut map = Vec::new();
    let mut buf = String::new();
    let mut pos = (0, 0);

    let mut row = 0;
    while reader.read_line(&mut buf)? != 0 {
        let mut map_line = Vec::new();
        for (col, c) in buf.trim().chars().enumerate() {
            match c {
                '.' => map_line.push(false),
                '#' => map_line.push(true),
                '^' => {
                    map_line.push(false);
                    pos = (row, col);
                }
                _ => {}
            }
        }
        map.push(map_line);
        row += 1;
        buf.clear();
    }

    Ok((
        Pos {
            r: pos.0,
            c: pos.1,
            d: Dir::N,
        },
        (map.len() - 1, map[0].len() - 1),
        map,
    ))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn next(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }
    fn prev(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<bool>>, pos: Pos) {
    for (r, map_line) in map.iter().enumerate() {
        let mut line = String::new();
        for (c, wall) in map_line.iter().enumerate() {
            if pos.r == r && pos.c == c {
                line.push(match pos.d {
                    Dir::E => '>',
                    Dir::S => 'v',
                    Dir::W => '<',
                    _ => '^',
                });
            } else if *wall {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
    println!("");
}

// using row, col for pos (y, x)
pub fn p1<R: BufRead>(reader: R) -> Result<i32, std::io::Error> {
    let (mut pos, bnd, map) = read_map(reader)?;
    let mut touched = HashSet::new();

    loop {
        let dir = pos.d;
        let (dist, end) = fast_travel(&map, &mut pos, bnd);
        for i in 0..dist {
            touched.insert(match dir {
                Dir::N => (pos.r + i, pos.c),
                Dir::E => (pos.r, pos.c - i),
                Dir::S => (pos.r - i, pos.c),
                Dir::W => (pos.r, pos.c + i),
            });
        }
        if end {
            break;
        }
    }

    Ok(touched.len() as i32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
    d: Dir,
}

fn fast_travel(map: &Vec<Vec<bool>>, pos: &mut Pos, bnd: (usize, usize)) -> (usize, bool) {
    let mut distance = 0;
    let gone;
    match pos.d {
        Dir::N => {
            while pos.r != 0 && !map[pos.r - 1][pos.c] {
                distance += 1;
                pos.r -= 1;
            }
            gone = pos.r == 0;
        }
        Dir::E => {
            while pos.c != bnd.1 && !map[pos.r][pos.c + 1] {
                distance += 1;
                pos.c += 1;
            }
            gone = pos.c == bnd.1;
        }
        Dir::S => {
            while pos.r != bnd.0 && !map[pos.r + 1][pos.c] {
                distance += 1;
                pos.r += 1;
            }
            gone = pos.r == bnd.0;
        }
        Dir::W => {
            while pos.c != 0 && !map[pos.r][pos.c - 1] {
                distance += 1;
                pos.c -= 1;
            }
            gone = pos.c == 0;
        }
    }
    pos.d = pos.d.next();
    (distance, gone)
}

fn creates_loop(
    map: &Vec<Vec<bool>>,
    mut pos: Pos,
    already_touched: &HashSet<Pos>,
    bnd: (usize, usize),
) -> bool {
    let mut touched = HashSet::new();
    loop {
        if fast_travel(map, &mut pos, bnd).1 {
            break;
        }
        if touched.contains(&pos) || already_touched.contains(&pos) {
            return true;
        } else {
            touched.insert(pos);
        }
    }
    false
}

pub fn p2<R: BufRead>(reader: R) -> Result<i32, std::io::Error> {
    let (start_pos, bnd, mut map) = read_map(reader)?;
    let mut pos = start_pos;
    let mut prev_pos;
    let mut touched: HashSet<Pos> = HashSet::new();
    let mut good_walls: HashSet<(usize, usize)> = HashSet::new();
    let mut bad_walls: HashSet<(usize, usize)> = HashSet::new();
    loop {
        prev_pos = pos;
        let (dist, gone) = fast_travel(&map, &mut pos, bnd);
        for i in 1..=dist {
            let change = match prev_pos.d {
                Dir::N => (prev_pos.r - i, prev_pos.c),
                Dir::E => (prev_pos.r, prev_pos.c + i),
                Dir::S => (prev_pos.r + i, prev_pos.c),
                Dir::W => (prev_pos.r, prev_pos.c - i),
            };
            if bad_walls.contains(&change) {
                continue;
            }
            map[change.0][change.1] = true;
            if creates_loop(&map, prev_pos, &touched, bnd) {
                good_walls.insert(change);
                //println!("HIT:")
            } else {
                bad_walls.insert(change);
            }
            //print_map(&map, prev_pos);
            map[change.0][change.1] = false;
        }
        touched.insert(Pos {
            d: pos.d.prev(),
            ..pos
        });
        touched.insert(pos);
        if gone {
            break;
        }
    }
    good_walls.remove(&(start_pos.r, start_pos.c));
    Ok(good_walls.len() as i32)
}
