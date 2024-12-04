use std::{collections::HashMap, io::BufRead, ops::AddAssign};


pub fn p1<R: BufRead>(mut reader: R) -> Result<i32,std::io::Error> {
    let mut buf=String::new();
    let mut arr1: Vec<i32> = vec![];
    let mut arr2: Vec<i32>=vec![];
    while reader.read_line(&mut buf)? != 0 {
        let (str1, str2) = buf.split_once(' ').unwrap();
        let num1:i32 = str1.parse().unwrap();
        arr1.push(num1);
        let num2:i32 = str2.trim().parse().unwrap();
        arr2.push(num2);
        buf.clear();
    }
    arr1.sort();
    arr2.sort();
    let result = arr1.into_iter().zip(arr2).fold(0, |init:i32, (a,b):(i32,i32)| {
        init + if a > b {
            a - b
        } else {
            b - a
        }
    });
    Ok(result)
}

pub fn p2<R: BufRead>(mut reader: R) -> Result<i32,std::io::Error> {
    let mut buf=String::new();
    let mut arr1: Vec<i32> = Vec::new();
    let mut c2 :HashMap<i32, i32> = HashMap::new();
    while reader.read_line(&mut buf)? != 0 {
        let (str1, str2) = buf.split_once(' ').unwrap();
        let num1:i32 = str1.parse().unwrap();
        arr1.push(num1);
        let num2:i32 = str2.trim().parse().unwrap();
        if c2.contains_key(&num2) {
            c2.get_mut(&num2).unwrap().add_assign(1);
        } else {
            c2.insert(num2, 1);
        }
        buf.clear();
    }
    let mut result = 0;
    for i in arr1 {
        result += i * c2.get(&i).unwrap_or(&0);
    }
    Ok(result)
}