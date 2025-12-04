use std::env;

mod challenges;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        match args[2].as_str() {
            "part1" => match args[1].as_str() {
                "day1" => {
                    challenges::day1::part1::exec();
                }
                "day2" => {
                    challenges::day2::part1::exec();
                }
                "day3" => {
                    challenges::day3::part1::exec();
                }
                "day4" => {
                    challenges::day4::part1::exec();
                }
                _ => {
                    eprintln!("invalid arugment for day")
                }
            },
            "part2" => match args[1].as_str() {
                "day1" => {
                    challenges::day1::part2::exec();
                }
                "day2" => {
                    challenges::day2::part2::exec();
                }
                "day3" => {
                    challenges::day3::part2::exec();
                }
                "day4" => {
                    challenges::day4::part2::exec();
                }
                _ => {
                    eprintln!("invalid arugment for day")
                }
            },
            _ => {
                eprintln!("invalid argument for part");
            }
        }
    }
}
