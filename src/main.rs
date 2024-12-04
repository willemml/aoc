mod aoc01;
mod aoc02;
mod aoc03;

macro_rules! run_day {
    ($aoc:ident, $day:expr) => {
        let file1 = std::fs::File::open(format!("aoc{:02}input", $day))?;
        let file2 = std::fs::File::open(format!("aoc{:02}input", $day))?;
        let reader1 = std::io::BufReader::new(file1);
        let reader2 = std::io::BufReader::new(file2);

        println!("Day {:02}: p1 = {}, p2 = {}", $day, $aoc::p1(reader1)?, $aoc::p2(reader2)?);
    };
}

fn main()-> Result<(), std::io::Error>{
    run_day!(aoc01, 1);
    run_day!(aoc02, 2);
    run_day!(aoc03, 3);
    Ok(())
}
