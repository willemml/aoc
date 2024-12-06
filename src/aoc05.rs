use std::{collections::HashMap, io::BufRead};

fn read_rules<R: BufRead>(reader: &mut R) -> Result<HashMap<i32, Vec<i32>>, std::io::Error> {
    let mut buf = String::new();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    while reader.read_line(&mut buf)? > 2 {
        let (req, num): (i32, i32) = {
            let (s1, s2) = buf.split_once('|').unwrap();
            (s1.trim().parse().unwrap(), s2.trim().parse().unwrap())
        };

        if rules.contains_key(&num) {
            rules.get_mut(&num).unwrap().push(req);
        } else {
            rules.insert(num, vec![req]);
        }
        buf.clear();
    }
    Ok(rules)
}

fn read_updates<R: BufRead>(reader: &mut R) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let mut updates = Vec::new();
    let mut buf = String::new();
    while reader.read_line(&mut buf)? != 0 {
        let mut update: Vec<i32> = Vec::new();
        for pagestr in buf.split(',') {
            let page: i32 = pagestr.trim().parse().unwrap();

            update.push(page);
        }
        updates.push(update);
        buf.clear();
    }
    Ok(updates)
}

fn is_good_update(rules: &HashMap<i32, Vec<i32>>, update: &[i32]) -> bool {
    let mut fail = false;
    for (i, page) in update.iter().enumerate() {
        if let Some(reql) = rules.get(&page) {
            for req in reql {
                if update.iter().position(|n| n == req) > Some(i) {
                    fail = true;
                    break;
                }
            }
            if fail {
                break;
            }
        }
    }
    !fail
}

pub fn p1<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let rules = read_rules(&mut reader)?;
    let updates = read_updates(&mut reader)?;
    let mut result = 0;

    for update in updates {
        if is_good_update(&rules, &update) {
            result += update[update.len() / 2];
        }
    }
    Ok(result)
}

pub fn p2<R: BufRead>(mut reader: R) -> Result<i32, std::io::Error> {
    let rules = read_rules(&mut reader)?;
    let updates = read_updates(&mut reader)?;
    let mut result = 0;

    for mut update in updates {
        if !is_good_update(&rules, &update) {
            while !is_good_update(&rules, &update) {
                let mut swap = Vec::new();
                for (i, page) in update.iter().enumerate() {
                    if let Some(reql) = rules.get(&page) {
                        for req in reql {
                            let pos = update.iter().position(|n| n == req);
                            if let Some(pos) = pos {
                                if pos > i {
                                    swap.push((pos, i));
                                    break;
                                }
                            }
                        }
                    }
                }
                for (i1, i2) in swap {
                    update.swap(i1, i2);
                }
            }
            result += update[update.len() / 2];
        }
    }
    Ok(result)
}
