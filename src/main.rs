mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;

macro_rules! run_day {
    ($aoc:ident, $day:expr, $file:expr) => {
        let file1 = std::fs::File::open(format!("aoc{:02}{}", $day, $file))?;
        let file2 = std::fs::File::open(format!("aoc{:02}{}", $day, $file))?;
        let reader1 = std::io::BufReader::new(file1);
        let reader2 = std::io::BufReader::new(file2);

        println!("Day {:02}: p1 = {}, p2 = {}", $day, $aoc::p1(reader1)?, $aoc::p2(reader2)?);
    };
    ($aoc:ident, $day:expr) => {
        run_day!($aoc, $day, "input");
    }
}

fn main()-> Result<(), std::io::Error>{
    run_day!(aoc01, 1);
    run_day!(aoc02, 2);
    run_day!(aoc03, 3);
    run_day!(aoc04, 4);
    run_day!(aoc05, 5);
    Ok(())
}
