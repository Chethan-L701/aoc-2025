use std::{collections::HashMap, env};

mod challenges;

#[derive(Debug, Clone, Copy)]
struct Challenge {
    part1: fn(),
    part2: fn(),
}

fn main() {
    #[rustfmt::skip]
    fn make_exucution_map() -> HashMap<&'static str, Challenge> {
        let mut exucution_map: HashMap<&str, Challenge> = HashMap::new();
        exucution_map.insert("day1", Challenge { part1: challenges::day1::part1::exec, part2: challenges::day1::part2::exec });
        exucution_map.insert("day2", Challenge { part1: challenges::day2::part1::exec, part2: challenges::day2::part2::exec });
        exucution_map.insert("day3", Challenge { part1: challenges::day3::part1::exec, part2: challenges::day3::part2::exec });
        exucution_map.insert("day4", Challenge { part1: challenges::day4::part1::exec, part2: challenges::day4::part2::exec });
        exucution_map.insert("day5", Challenge { part1: challenges::day5::part1::exec, part2: challenges::day5::part2::exec });
        exucution_map.insert("day6", Challenge { part1: challenges::day6::part1::exec, part2: challenges::day6::part2::exec });

        exucution_map
    }
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let day = args[1].trim();
        let part = args[2].trim();

        let map = make_exucution_map();
        match part {
            "part1" => (map.get(day).unwrap().part1)(),
            "part2" => (map.get(day).unwrap().part2)(),
            _ => {}
        }
    }
}
