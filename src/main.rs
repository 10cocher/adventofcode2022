use regex::Regex;
use std::fs;

#[derive(Debug)]
struct PaperRock {
    me: char,
    you: char,
}

impl PaperRock {
    fn score_my_choice(&self) -> usize {
        match &self.me {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unreachable!(),
        }
    }

    fn score_confrontation(&self) -> usize {
        match (&self.me, &self.you) {
            ('X', 'B') | ('Y', 'C') | ('Z', 'A') => 0,
            ('X', 'C') | ('Y', 'A') | ('Z', 'B') => 6,
            _ => 3,
        }
    }

    fn score_total(&self) -> usize {
        self.score_my_choice() + self.score_confrontation()
    }
}

fn parse_input(result: char, you: char) -> PaperRock {
    let me: char = match (result, you) {
        ('X', 'A') => 'Z',
        ('X', 'B') => 'X',
        ('X', 'C') => 'Y',
        ('Y', 'A') => 'X',
        ('Y', 'B') => 'Y',
        ('Y', 'C') => 'Z',
        ('Z', 'A') => 'Y',
        ('Z', 'B') => 'Z',
        ('Z', 'C') => 'X',
        _ => unreachable!(),
    };
    PaperRock { you, me }
}

fn day02() {
    let file_path = "./inputs/day02/input.txt";

    println!("Reading {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    //let mut count = 0;
    let _data_part_1: usize = contents
        .lines()
        .map(|line| {
            let mut toto = line.chars(); //split_once(" ").unwrap();
            let a = toto.next().unwrap(); //(toto.0, toto.1)
            let b = toto.nth(1).unwrap();
            let rps = PaperRock { you: a, me: b };
            //let toto: Vec<&str> = line.split(" ").map(|x| x).collect();
            rps.score_total()
            //toto[0]
        })
        .sum();

    let _data_part_2: usize = contents
        .lines()
        .map(|line| {
            let mut toto = line.chars();
            let a = toto.nth(0).unwrap();
            let b = toto.nth(1).unwrap();
            let rps = parse_input(b, a);
            // println!("{}, {}, {:?}, {}", a, b, rps, rps.score_total());
            rps.score_total()
        })
        .sum();
    // println!("{:?} {:?}", &data_part_1, &data_part_2);
    // let row = contents[0]
    //for number in 1..4 {
    //    println!("{number}!");
    //}

    //println!("Content of the file:\n{row}");
}

fn find_common_character(first: &str, second: &str) -> char {
    for candidate_char in first.chars() {
        if second.contains(candidate_char) {
            return candidate_char;
        }
    }
    '4'
}

fn find_all_common_characters(first: &str, second: &str) -> String {
    let mut result: String = String::from("");
    for candidate_char in first.chars() {
        if second.contains(candidate_char) {
            result.push(candidate_char)
        }
    }
    result
}

fn character_to_priority(&c: &char) -> usize {
    let mut order: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let order_upper: &str = &order.clone().to_uppercase();
    order.push_str(order_upper);
    order.find(c).unwrap() + 1
}

fn day03() {
    let file_path = "./inputs/day03/input.txt";
    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file of day 3");
    let rucksacks = contents.lines();
    let res_part1: usize = rucksacks
        .map(|line: &str| -> usize {
            let n_chars = line.len();
            // println!("There are {n_chars} characters in this line");
            let compartiment_size = n_chars / 2;
            let common_char =
                find_common_character(&line[0..compartiment_size], &line[compartiment_size..]);
            // println!("The common char is {common_char}");
            let priority = character_to_priority(&common_char);
            //println!("The priority is {priority}");
            priority
        })
        .sum();
    println!("The result for part 1 of day 03 is {res_part1}");

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file of day 3");
    let rucksacks = contents.lines();
    // Prepare three variable
    let mut elf0: &str = "";
    let mut elf1: &str = "";
    let mut elf2: &str;
    let mut res_part2: usize = 0;
    // Loop over each rucksack
    for (i, indiv_line) in rucksacks.enumerate() {
        // println!("Line {i} is len {}", indiv_line);
        if i % 3 == 0 {
            elf0 = indiv_line;
        } else if i % 3 == 1 {
            elf1 = indiv_line;
        } else if i % 3 == 2 {
            elf2 = indiv_line;
            let common_elf0_elf1 = find_all_common_characters(elf0, elf1);
            // println!("common to elf0 and elf1 is {common_elf0_elf1}");
            let badge = find_common_character(&common_elf0_elf1, elf2);
            // println!("badge is {badge}");
            res_part2 = res_part2 + character_to_priority(&badge);
        }
    }
    println!("The result for part 2 of day 03 is {res_part2}");
}

fn day04() {
    let file_path = "./inputs/day04/input.txt";
    // println!("Reading {file_path}");
    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file of day 4");
    let re = match Regex::new("^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)$") {
        Ok(result) => result,
        Err(error) => {
            panic!("Problem constructing the regex {}", error);
        }
    };
    let mut res_part1: usize = 0;
    let mut res_part2: usize = 0;
    // Loop over each line
    for (_, line) in contents.lines().enumerate() {
        //let res_both_parts: (usize, usize) = contents
        //    .lines()
        //    .map(|line: &str| -> (usize, usize) {
        let match_results = match re.captures(line) {
            Some(match_result) => match_result,
            None => panic!("I did not find match"),
        };
        let beg_range0: isize = match_results[1].parse().unwrap();
        let end_range0: isize = match_results[2].parse().unwrap();
        let beg_range1: isize = match_results[3].parse().unwrap();
        let end_range1: isize = match_results[4].parse().unwrap();

        let res_one_line_part1: usize =
            if (beg_range1 - beg_range0) * (end_range0 - end_range1) >= 0 {
                1
            } else {
                0
            };
        let res_one_line_part2: usize =
            if (beg_range1 - beg_range0) * (end_range0 - end_range1) >= 0 {
                1
            } else if (end_range0 >= beg_range1) & (end_range1 > beg_range0) {
                1
            } else if (end_range1 >= beg_range0) & (end_range0 > beg_range1) {
                1
            } else {
                0
            };
        // println!("The group is: {beg_range0}, {end_range0}, {beg_range1}, {end_range1}, {res_one_line}");
        res_part1 += res_one_line_part1;
        res_part2 += res_one_line_part2;
    }
    //.sum();
    println!("The result for part 1 of day 04 is {res_part1}");
    println!("The result for part 2 of day 04 is {res_part2}");
}

fn main() {
    println!("Hello, world!");
    day02();
    day03();
    day04();
}
