#![allow(clippy::needless_return)]

mod utils {
    use std::fs;
    pub fn read_file_lines(path: &str) -> Vec<String> {
        let f = fs::read_to_string(path);
        let v: Vec<String> = f.unwrap().lines().map(|x| x.to_string()).collect();

        return v;
    }
    pub fn is_even(num: i32) -> bool {
        return if num % 2 == 0 { true } else { false };
    }
}
#[allow(dead_code)]
pub mod day1 {
    pub mod part1 {
        use crate::challenges::utils;
        pub fn exec() {
            let data = utils::read_file_lines("./data/day1.txt");
            let mut dial = 50;
            let mut password = 0;

            for l in data {
                let direction = l.chars().nth(0).unwrap();
                let times: i32 = l[1..].parse().unwrap();

                match direction {
                    'L' => {
                        dial = (dial - times) % 100;
                        if dial < 0 {
                            dial = 100 + dial;
                        }
                    }

                    'R' => {
                        dial = (dial + times) % 100;
                    }

                    _ => {}
                }

                if dial == 0 {
                    password += 1;
                }

                println!(
                    "{{direction : {}, times : {}, current_pos: {} }}",
                    direction, times, dial
                );
            }

            println!("password : {}", password);
        }
    }

    pub mod part2 {
        use crate::challenges::utils;
        pub fn exec() {
            let data = utils::read_file_lines("./data/day1.txt");
            let mut dial = 50;
            let mut password = 0;

            for l in data {
                let direction = l.chars().nth(0).unwrap();
                let mut times: i32 = l[1..].parse().unwrap();

                let rev: i32 = times / 100;

                times = times % 100;

                let mut across_zero: bool = false;

                match direction {
                    'L' => {
                        let pre_turn = dial;
                        dial = (dial - times) % 100;
                        if dial < 0 {
                            dial = 100 + dial;
                            if pre_turn != 0 && dial != 0 {
                                across_zero = true;
                            }
                        }
                    }

                    'R' => {
                        let pre_turn = dial;
                        dial = dial + times;
                        if dial >= 100 {
                            dial = dial % 100;
                            if pre_turn != 0 && dial != 0 {
                                across_zero = true;
                            }
                        }
                    }
                    _ => {}
                }

                password += rev + if across_zero { 1 } else { 0 } + if dial == 0 { 1 } else { 0 };

                println!(
                    "{{direction : {}, times : {}, rev : {}, current_pos: {}, across_zero : {} }}",
                    direction, times, rev, dial, across_zero
                );
            }
            println!("password : {}", password);
        }
    }
}

pub mod day2 {
    pub mod part1 {
        use crate::challenges::utils;

        pub fn is_valid_id(id: i64) -> bool {
            let id = id.to_string();
            if utils::is_even(id.len() as i32) {
                let mid = id.len() / 2;
                if id[..mid] == id[mid..] {
                    return false;
                }
            }
            return true;
        }
        pub fn exec() {
            let data = utils::read_file_lines("./data/day2.txt");
            let data: Vec<String> = data
                .iter()
                .nth(0)
                .unwrap()
                .split(',')
                .map(|x| x.to_string())
                .collect();
            let mut bounds: Vec<(i64, i64)> = vec![];
            for d in data {
                let d: Vec<&str> = d.split("-").map(|x| x.trim()).collect();
                bounds.push((d[0].parse().unwrap(), d[1].parse().unwrap()));
            }
            let mut invalid_id_sum: i64 = 0;
            for bound in bounds {
                println!("{{ first_id : {}, last_id : {} }}", bound.0, bound.1);
                for id in bound.0..=bound.1 {
                    if !is_valid_id(id) {
                        invalid_id_sum += id;
                    }
                }
            }
            println!("invalid id sum : {}", invalid_id_sum);
        }
    }
    pub mod part2 {
        use crate::challenges::utils;
        fn is_repeated(id: &String, step: usize) -> bool {
            let parts: Vec<char> = id.chars().collect();
            let parts_chunks: Vec<Vec<char>> = parts.chunks(step).map(|v| v.to_vec()).collect();
            let parts_slices: Vec<String> = parts_chunks
                .iter()
                .map(|v| v.iter().fold(String::new(), |acc, c| acc + &c.to_string()))
                .collect();
            for i in 0..parts_slices.len() - 1 {
                if parts_slices[i] != parts_slices[i + 1] {
                    return false;
                }
            }

            return true;
        }

        fn is_valid_id(id: i64) -> bool {
            let id = id.to_string();
            for i in 1..(id.len() / 2) + 1 {
                if is_repeated(&id, i) {
                    return false;
                }
            }
            return true;
        }
        pub fn exec() {
            let data = utils::read_file_lines("./data/day2.txt");
            let data: Vec<String> = data
                .iter()
                .nth(0)
                .unwrap()
                .split(',')
                .map(|x| x.to_string())
                .collect();
            let mut bounds: Vec<(i64, i64)> = vec![];
            for d in data {
                let d: Vec<&str> = d.split("-").map(|x| x.trim()).collect();
                bounds.push((d[0].parse().unwrap(), d[1].parse().unwrap()));
            }
            let mut invalid_id_sum: i64 = 0;
            for bound in bounds {
                println!("{{ first_id : {}, last_id : {} }}", bound.0, bound.1);
                for id in bound.0..=bound.1 {
                    if !is_valid_id(id) {
                        invalid_id_sum += id;
                    }
                }
            }
            println!("invalid id sum : {}", invalid_id_sum);
        }
    }
}

pub mod day3 {
    pub mod part1 {
        use crate::challenges::utils;

        pub fn get_joltage(s: &String) -> i32 {
            let mut joltage: String = String::new();
            let mut slice_at: usize = 0;
            {
                let mut max: i32 = 0;
                let len = s.len();
                let batteries: Vec<char> = s[..len - 1].chars().collect();
                for i in 0..batteries.len() {
                    let rating: i32 = batteries[i].to_digit(10).unwrap() as i32;
                    if rating > max {
                        max = rating;
                        slice_at = i;
                    }
                }
                joltage += &max.to_string();
            }
            {
                let mut max: i32 = 0;
                let batteries: Vec<char> = s[slice_at + 1..].chars().collect();
                for i in 0..batteries.len() {
                    let rating: i32 = batteries[i].to_digit(10).unwrap() as i32;
                    if rating > max {
                        max = rating;
                    }
                }
                joltage += &max.to_string();
            }
            return joltage.parse().unwrap();
        }

        pub fn exec() {
            let input = utils::read_file_lines("./data/day3.txt");
            let mut total_joltage: i64 = 0;
            for set in input {
                total_joltage += get_joltage(&set) as i64;
            }
            println!("total_joltage : {} ", total_joltage);
        }
    }
    pub mod part2 {

        use crate::challenges::utils;

        #[derive(Debug)]
        struct Battery {
            val: i32,
            pos: usize,
        }

        fn get_max(s: &String) -> Battery {
            let mut max = 0;
            let mut pos: usize = 0;

            {
                let batteries: Vec<char> = s.chars().collect();
                for i in 0..batteries.len() {
                    let rating: i32 = batteries[i].to_digit(10).unwrap() as i32;
                    if rating > max {
                        max = rating;
                        pos = i;
                    }
                }
            }
            let battery = Battery { val: max, pos };
            return battery;
        }

        fn get_joltage(s: &String) -> u64 {
            let mut joltage = String::new();
            let mut current_pos: usize = 0;

            for i in 1..=12 {
                let max = get_max(&s[current_pos..s.len() - (12 - i)].to_string());
                current_pos += max.pos + 1;
                println!(
                    "{{ max battery rating : {}, position in set : {} }}",
                    max.val, current_pos
                );
                joltage += &max.val.to_string();
            }

            println!("joltage : {}", joltage);

            return joltage.parse().unwrap();
        }

        pub fn exec() {
            let input = utils::read_file_lines("./data/day3.txt");
            let mut total_joltage: u64 = 0;
            for set in input {
                total_joltage += get_joltage(&set);
            }
            println!("total_joltage : {} ", total_joltage);
        }
    }
}

pub mod day4 {
    pub mod part1 {

        use crate::challenges::utils;

        fn arrange(d: Vec<String>) -> Vec<Vec<char>> {
            let mut data: Vec<Vec<char>> = vec![];
            for l in d {
                let mut rolls: Vec<char> = l.chars().collect();
                rolls.insert(0, '.');
                rolls.push('.');
                data.push(rolls);
            }
            let pad: String = data[0].iter().fold(String::new(), |acc, _| acc + ".");
            data.insert(0, pad.chars().collect());
            data.push(pad.chars().collect());
            for d in &data {
                println!(
                    "{}",
                    d.iter().fold(String::new(), |acc, x| acc + &x.to_string())
                );
            }
            return data;
        }

        fn transform(d: Vec<Vec<char>>) -> u64 {
            let mut available: u64 = 0;
            let mut transformed: Vec<Vec<char>> = vec![];
            for i in 1..d.len() - 1 {
                let mut row: Vec<char> = vec![];
                for j in 1..d[0].len() - 1 {
                    let top = if d[i - 1][j - 1] == '@' { 1 } else { 0 }
                        + if d[i - 1][j] == '@' { 1 } else { 0 }
                        + if d[i - 1][j + 1] == '@' { 1 } else { 0 };

                    let mid = if d[i][j - 1] == '@' { 1 } else { 0 }
                        + if d[i][j + 1] == '@' { 1 } else { 0 };

                    let bottom = if d[i + 1][j - 1] == '@' { 1 } else { 0 }
                        + if d[i + 1][j] == '@' { 1 } else { 0 }
                        + if d[i + 1][j + 1] == '@' { 1 } else { 0 };

                    if ((top + mid + bottom) < 4) && d[i][j] != '.' {
                        available += 1;
                        row.push('x');
                    } else {
                        row.push(d[i][j]);
                    }
                }
                transformed.push(row);
            }

            for d in &transformed {
                println!(
                    "{}",
                    d.iter().fold(String::new(), |acc, x| acc + &x.to_string())
                );
            }
            return available;
        }

        pub fn exec() {
            let data = arrange(utils::read_file_lines("./data/day4.txt"));
            println!("available : {}", transform(data));
        }
    }
    pub mod part2 {
        use crate::challenges::utils;

        fn arrange(d: Vec<String>) -> Vec<Vec<char>> {
            let mut data: Vec<Vec<char>> = vec![];
            for l in d {
                let mut rolls: Vec<char> = l.chars().collect();
                rolls.insert(0, '.');
                rolls.push('.');
                data.push(rolls);
            }
            let pad: String = data[0].iter().fold(String::new(), |acc, _| acc + ".");
            data.insert(0, pad.chars().collect());
            data.push(pad.chars().collect());
            return data;
        }

        fn transform(d: Vec<Vec<char>>) -> (u64, Vec<String>) {
            let mut available: u64 = 0;
            let mut transformed: Vec<Vec<char>> = vec![];
            for i in 1..d.len() - 1 {
                let mut row: Vec<char> = vec![];
                for j in 1..d[0].len() - 1 {
                    let mut is_removable: bool = false;
                    let top = if d[i - 1][j - 1] == '@' { 1 } else { 0 }
                        + if d[i - 1][j] == '@' { 1 } else { 0 }
                        + if d[i - 1][j + 1] == '@' { 1 } else { 0 };

                    let mid = if d[i][j - 1] == '@' { 1 } else { 0 }
                        + if d[i][j + 1] == '@' { 1 } else { 0 };

                    let bottom = if d[i + 1][j - 1] == '@' { 1 } else { 0 }
                        + if d[i + 1][j] == '@' { 1 } else { 0 }
                        + if d[i + 1][j + 1] == '@' { 1 } else { 0 };

                    if ((top + mid + bottom) < 4) && d[i][j] != '.' && d[i][j] != 'x' {
                        is_removable = true;
                        available += 1;
                    }
                    if is_removable {
                        if d[i][j] == 'x' {
                            row.push('.');
                        } else {
                            row.push('x');
                        }
                    } else {
                        row.push(d[i][j]);
                    }
                }
                transformed.push(row);
            }

            let mut transformed_: Vec<String> = vec![];

            for d in &transformed {
                transformed_.push(d.iter().fold(String::new(), |acc, x| acc + &x.to_string()));
            }
            return (available, transformed_);
        }
        fn total_removable(data: Vec<String>) -> u64 {
            let mut data = data.clone();
            let mut total: u64 = 0;
            loop {
                let set: (u64, Vec<String>) = transform(arrange(data));
                data = set.1;
                total += set.0;

                println!("\nremovable: {}", set.0);
                for d in &data {
                    println!("{}", d);
                }

                if set.0 == 0 {
                    break;
                }
            }
            return total;
        }
        pub fn exec() {
            let data = utils::read_file_lines("./data/day4.txt");
            println!("total removables : {}", total_removable(data));
        }
    }
}

pub mod day5 {
    #[derive(Debug, Copy, Clone)]
    struct Range {
        lower: u64,
        upper: u64,
    }
    fn get_ranges(data: Vec<String>) -> Vec<Range> {
        let mut ranges: Vec<Range> = vec![];
        for d in data {
            let r: Vec<&str> = d.split('-').collect();
            ranges.push(Range {
                lower: r[0].parse().unwrap(),
                upper: r[1].parse().unwrap(),
            });
        }
        return ranges;
    }
    pub mod part1 {
        use super::Range;
        use crate::challenges::utils;
        use super::get_ranges;

        fn get_ingredients(data: Vec<String>) -> Vec<u64> {
            let mut ingredients: Vec<u64> = vec![];
            for d in data {
                ingredients.push(d.parse().unwrap());
            }
            return ingredients;
        }

        fn get_fresh(ingredients: Vec<u64>, ranges: Vec<Range>) -> u64 {
            let mut fresh_ingredients: u64 = 0;

            for i in ingredients {
                for range in &ranges {
                    if i >= range.lower && i <= range.upper {
                        fresh_ingredients += 1;
                        break;
                    }
                }
            }

            return fresh_ingredients;
        }

        pub fn exec() {
            let ranges = get_ranges(utils::read_file_lines("./data/day5-ranges.txt"));
            let ingredients = get_ingredients(utils::read_file_lines("./data/day5-ing.txt"));
            println!("fresh ingredients : {}", get_fresh(ingredients, ranges));
        }
    }
    pub mod part2 {
        use super::Range;
        use crate::challenges::utils;
        use super::get_ranges;

        use std::cmp::Ordering;

        fn reduce(ranges: Vec<Range>) -> Vec<Range> {
            let mut reduced: Vec<Range> = vec![];
            for r in ranges {
                if let Some(last) = reduced.last_mut() {
                    if r.lower <= last.upper {
                        last.upper = last.upper.max(r.upper);
                    } else {
                        reduced.push(r);
                    }
                } else {
                    reduced.push(r);
                }
            }
            return reduced;
        }
        fn get_total_fresh(ranges: Vec<Range>) -> u64 {
            let mut fresh_ingredients: u64 = 0;

            for range in &ranges {
                fresh_ingredients += range.upper  - range.lower + 1;
            }

            return fresh_ingredients;
        }

        pub fn exec() {
            let mut ranges = get_ranges(utils::read_file_lines("./data/day5-ranges.txt"));
            ranges.sort_by(|a, b| {
                if a.lower < b.lower {
                    Ordering::Less
                } else if a.lower == b.lower {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            });
            for (i, r) in ranges.iter().enumerate() {
                println!("{{ {} :  {:?}; {} }}", i + 1, r, r.upper - r.lower);
            }
            println!("\n\n");
            let reduced = reduce(ranges);
            for (i, r) in reduced.iter().enumerate() {
                println!("{{ {} :  {:?}; {} }}", i + 1, r, r.upper - r.lower);
            }
            println!("total fresh ingredients : {}", get_total_fresh(reduced));
        }
    }
}
