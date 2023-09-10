use array2d::Array2D;
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
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

fn day05() {
    let file_path = "./inputs/day05/input0.txt";
    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file of day 5");
    let re = match Regex::new("^move ([0-9]+) from ([0-9]+) to ([0-9]+)$") {
        Ok(result) => result,
        Err(error) => panic!("Problem constructing the regex {}", error),
    };
    let mut initialization_finished: bool = false;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for (_, line) in contents.lines().enumerate() {
        if !initialization_finished {
            let n_cols: usize = line.len();
            // println!("There are {n_cols} in {}", line);
            if n_cols == 0 {
                initialization_finished = true;
                // reverse all individual stacks
                stacks.iter_mut().for_each(|stack| stack.reverse());
                // println!("stacks {:?}", stacks);
                continue;
            }
            if (line.as_bytes()[0] as char == ' ') & (line.as_bytes()[1] as char != ' ') {
                continue;
            }
            if !initialization_finished {
                let trimed_line: &str = line.trim_end();
                let n_cols_trim: usize = trimed_line.len();
                let n_stacks_this_line: usize = (n_cols_trim + 1) / 4;
                for i_stack in 0..n_stacks_this_line {
                    let char_pos: usize = 4 * i_stack + 1;
                    let byte_char_pos: u8 = trimed_line.as_bytes()[char_pos];
                    let new_char: char = byte_char_pos as char;
                    if stacks.len() < i_stack + 1 {
                        let mut new_vec = Vec::new();
                        if new_char != ' ' {
                            new_vec.push(new_char);
                        }
                        stacks.push(new_vec);
                    } else {
                        if new_char != ' ' {
                            stacks[i_stack].push(new_char);
                        }
                    }
                }
            }
        } else {
            let match_results = match re.captures(line) {
                Some(match_result) => match_result,
                None => panic!("I did not find match"),
            };
            let height: isize = match_results[1].parse().unwrap();
            let origin: usize = match_results[2].parse().unwrap();
            let destination: usize = match_results[3].parse().unwrap();
            let mut temp_stack: Vec<char> = Vec::new();
            for _ in 0..height {
                // println!("stacks {:?}", stacks);
                // println!("height={height}, origin={origin}, destination={destination}");
                let char_to_move: char = stacks[origin - 1].pop().unwrap();
                temp_stack.push(char_to_move);
                // stacks[destination - 1].push(char_to_move);
            }
            for _ in 0..height {
                stacks[destination - 1].push(temp_stack.pop().unwrap());
            }
        }
    }
    // println!("stacks {:?}", stacks);
    let mut res_part1: String = String::from("");
    for stack in stacks.iter_mut() {
        let char_to_append = stack.pop().unwrap();
        res_part1.push(char_to_append);
    }
    println!("The result for part 2 of day 05 is {res_part1}");
}

fn find_result_day06(word: &str, depth: usize) -> usize {
    let mut result: usize = 0;
    let mut candidate: usize = 0;
    let last_candidate: usize = word.len() - depth;
    // println!("depth={depth}, last_candidate={last_candidate}");
    while candidate <= last_candidate {
        let mut ok_candidate: bool = true;
        for level in 1..depth {
            let mut ok_level: bool = true;
            // println!("level {level}");
            for a in candidate..candidate + depth - level {
                let b = a + level;
                let chara = &word[a..=a];
                let charb = &word[b..=b];
                if chara == charb {
                    ok_level = false;
                    break;
                }
                // println!("Compare chars at pos {a} and {b}, {chara}, {charb}")
            }
            if !ok_level {
                ok_candidate = false;
                break;
            }
        }
        if ok_candidate {
            result = candidate + depth;
            break;
        } else {
            candidate = candidate + 1;
        }
    }
    result
}

fn day06() {
    let file_path: &str = "./inputs/day06/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let buffer = contents.trim();
    let res_part1: usize = find_result_day06(&buffer, 4);
    println!("The result for part 1 of day 06 is {res_part1}");
    let res_part2: usize = find_result_day06(&buffer, 14);
    println!("The result for part 2 of day 06 is {res_part2}");
    //println!("'{buffer}'");
}

fn day07() {
    let file_path: &str = "./inputs/day07/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    //
    let mut active_folders: Vec<&str> = Vec::new();
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();
    let mut files: Vec<String> = Vec::new();
    //
    for line in lines {
        if &line[0..4] == "$ cd" {
            if &line[5..6] == "." {
                active_folders.pop();
            } else {
                let new_folder: &str = &line[5..];
                active_folders.push(new_folder);
            }
        } else if (&line[0..3] != "dir") & (&line[0..4] != "$ ls") {
            let filedata: Vec<&str> = line.split_whitespace().collect();
            let filesize: usize = filedata[0].parse().unwrap();
            let filename: &str = filedata[1];
            let mut absolute_path = String::from("");
            for folder in &active_folders {
                absolute_path.push_str(folder);
                absolute_path.push_str("/");
                let size_count = folder_sizes.entry(absolute_path.clone()).or_insert(0);
                *size_count += filesize;
            }
            absolute_path.push_str(filename);
            if files.contains(&absolute_path) {
                println!("This one was already counted {}", absolute_path)
            } else {
                files.push(absolute_path);
            }
        }
    }
    let mut res_part1: usize = 0;
    for (_, folder_size) in &folder_sizes {
        if folder_size <= &100_000 {
            res_part1 = res_part1 + folder_size;
        }
    }
    println!("The result for part 1 of day 07 is {res_part1}");
    let used_space: &usize = folder_sizes.get("//").unwrap();
    // println! {"Used space is {used_space}"}
    let free_space: usize = 70_000_000 - used_space;
    // println! {"Free space is {free_space}"}
    let space_to_free: usize = 30_000_000 - free_space;
    // println!("Need to free {space_to_free}");
    let mut res_part2: usize = 70_000_000;
    let mut all: Vec<usize> = Vec::new();
    for (_, folder_size) in folder_sizes {
        all.push(folder_size.clone());
        if (folder_size < res_part2) & (folder_size >= space_to_free) {
            res_part2 = folder_size;
        }
    }
    all.sort();
    // println!("{:?}", all);
    println!("The result for part 2 of day 07 is {res_part2}");
}

fn day08() {
    let file_path: &str = "./inputs/day08/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();
    //
    let n_rows: usize = lines.clone().count();
    let line = lines.clone().next().unwrap();
    let n_cols: usize = line.len();
    // Initialize from file
    let mut array: Array2D<u8> = Array2D::filled_with(0, n_rows, n_cols);
    for (i_row, line) in lines.enumerate() {
        for (i_col, character) in line.chars().enumerate() {
            let value: u8 = character.to_string().parse().unwrap();
            let _result = array.set(i_row, i_col, value);
        }
    }
    //
    let mut ok: Array2D<bool> = Array2D::filled_with(false, n_rows, n_cols);
    // Check from top
    let mut row_ok: Vec<u8> = vec![0; n_cols];
    for i_col in 0..n_cols {
        row_ok[i_col] = array.get(0, i_col).unwrap().clone();
        let _result = ok.set(0, i_col, true);
    }
    for i_row in 1..(n_rows - 1) {
        for i_col in 0..n_rows {
            let tree: u8 = array.get(i_row, i_col).unwrap().clone();
            let visible_tree: bool = tree > row_ok[i_col];
            row_ok[i_col] = cmp::max(row_ok[i_col], tree);
            ok.get_mut(i_row, i_col).map(|x| *x = *x | &visible_tree);
        }
    }
    // Check from bottom
    let mut row_ok: Vec<u8> = vec![0; n_cols];
    for i_col in 0..n_cols {
        row_ok[i_col] = array.get(n_rows - 1, i_col).unwrap().clone();
        let _result = ok.set(n_rows - 1, i_col, true);
    }
    for i_row in (1..(n_rows - 1)).rev() {
        for i_col in 0..n_rows {
            let tree: u8 = array.get(i_row, i_col).unwrap().clone();
            let visible_tree: bool = tree > row_ok[i_col];
            row_ok[i_col] = cmp::max(row_ok[i_col], tree);
            ok.get_mut(i_row, i_col).map(|x| *x = *x | &visible_tree);
        }
    }
    // Check from right
    let mut col_ok: Vec<u8> = vec![0; n_rows];
    for i_row in 0..n_rows {
        col_ok[i_row] = array.get(i_row, n_cols - 1).unwrap().clone();
        let _result = ok.set(i_row, n_cols - 1, true);
    }
    for i_col in (1..(n_cols - 1)).rev() {
        for i_row in 0..n_rows {
            let tree: u8 = array.get(i_row, i_col).unwrap().clone();
            let visible_tree: bool = tree > col_ok[i_row];
            col_ok[i_row] = cmp::max(col_ok[i_row], tree);
            ok.get_mut(i_row, i_col).map(|x| *x = *x | &visible_tree);
        }
    }
    // Check from left
    let mut col_ok: Vec<u8> = vec![0; n_rows];
    for i_row in 0..n_rows {
        col_ok[i_row] = array.get(i_row, 0).unwrap().clone();
        let _result = ok.set(i_row, 0, true);
    }
    for i_col in 1..(n_cols - 1) {
        for i_row in 0..n_rows {
            let tree: u8 = array.get(i_row, i_col).unwrap().clone();
            let visible_tree: bool = tree > col_ok[i_row];
            col_ok[i_row] = cmp::max(col_ok[i_row], tree);
            ok.get_mut(i_row, i_col).map(|x| *x = *x | &visible_tree);
        }
    }

    let mut res_part1: usize = 0;
    for i_row in 0..n_rows {
        let mut word = String::from("");
        for i_col in 0..n_cols {
            if *ok.get(i_row, i_col).unwrap() {
                word.push_str("1");
                res_part1 = res_part1 + 1;
            } else {
                word.push_str("0");
            }
        }
        // println!("{}", word);
    }
    println!("The result for part 1 of day 08 is {res_part1}");

    // Part 2
    let mut res_part2: usize = 0;
    for i_row in 0..n_rows {
        for i_col in 0..n_cols {
            let tree: u8 = array.get(i_row, i_col).unwrap().clone();
            // bottom
            let mut bottom: usize = 0;
            for i_row2 in (i_row + 1)..n_rows {
                bottom = bottom + 1;
                if array.get(i_row2, i_col).unwrap() >= &tree {
                    break;
                }
            }
            // top
            let mut top: usize = 0;
            for i_row2 in (0..i_row).rev() {
                top = top + 1;
                if array.get(i_row2, i_col).unwrap() >= &tree {
                    break;
                }
            }
            // left
            let mut left: usize = 0;
            for i_col2 in (0..i_col).rev() {
                left = left + 1;
                if array.get(i_row, i_col2).unwrap() >= &tree {
                    break;
                }
            }
            // right
            let mut right: usize = 0;
            for i_col2 in i_col + 1..n_cols {
                right = right + 1;
                if array.get(i_row, i_col2).unwrap() >= &tree {
                    break;
                }
            }
            let view: usize = bottom * top * left * right;
            // println!("row={i_row} col={i_col} view={view} bottom={bottom} top={top} left={left} right={right}");
            if view > res_part2 {
                res_part2 = view
            }
        }
    }
    println!("The result for part 2 of day 08 is {res_part2}");
}

fn day09_update_head(coord: &mut (isize, isize), dir: &str) {
    let update: (isize, isize) = match &dir {
        &"R" => (1, 0),
        &"L" => (-1, 0),
        &"U" => (0, 1),
        &"D" => (0, -1),
        _ => unreachable!(),
    };
    coord.0 = coord.0 + update.0;
    coord.1 = coord.1 + update.1;
}

fn day09_update_tail(coord_tail: &mut (isize, isize), coord_head: &(isize, isize)) {
    // println!("coord_tail={coord_tail:?}, coord_head={coord_head:?}");
    let delta: (isize, isize) = (coord_head.0 - coord_tail.0, coord_head.1 - coord_tail.1);
    let update: (isize, isize) = match &delta {
        &(0, 0) => (0, 0),
        &(1, 0) => (0, 0),
        &(-1, 0) => (0, 0),
        &(0, 1) => (0, 0),
        &(0, -1) => (0, 0),
        &(1, 1) => (0, 0),
        &(1, -1) => (0, 0),
        &(-1, 1) => (0, 0),
        &(-1, -1) => (0, 0),
        &(2, 0) => (1, 0),
        &(-2, 0) => (-1, 0),
        &(0, 2) => (0, 1),
        &(0, -2) => (0, -1),
        &(1, 2) => (1, 1),
        &(2, 1) => (1, 1),
        &(2, -1) => (1, -1),
        &(1, -2) => (1, -1),
        &(-1, -2) => (-1, -1),
        &(-2, -1) => (-1, -1),
        &(-2, 1) => (-1, 1),
        &(-1, 2) => (-1, 1),
        &(2, 2) => (1, 1),
        &(2, -2) => (1, -1),
        &(-2, -2) => (-1, -1),
        &(-2, 2) => (-1, 1),
        _ => unreachable!(),
    };
    coord_tail.0 = coord_tail.0 + update.0;
    coord_tail.1 = coord_tail.1 + update.1;
}

fn day09() {
    let file_path: &str = "./inputs/day09/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    //
    let n_knots: u8 = 9;
    let last_knot: u8 = n_knots - 1;
    let mut coord_head: (isize, isize) = (0, 0);
    let mut coord_knots: HashMap<u8, (isize, isize)> = HashMap::new();
    for i_knot in 0..n_knots {
        coord_knots.insert(i_knot, (0, 0));
    }
    let mut visited: HashMap<(isize, isize), usize> = HashMap::new();
    let coord_tail: (isize, isize) = coord_knots.get(&last_knot).unwrap().clone();
    visited.insert(coord_tail, 1);
    //
    for line in contents.lines() {
        let head_dir: &str = &line[0..1];
        let head_offset: usize = line[2..].parse().expect("Could not conver to usize");
        for _i in 0..head_offset {
            day09_update_head(&mut coord_head, head_dir);
            // println!("==========================");
            // println!("New coord_head={coord_head:?}");
            // println!("Knot 0");
            let mut coord_to_update = coord_knots.entry(0).or_insert((0, 0));
            day09_update_tail(&mut coord_to_update, &coord_head);
            for i_knot in 1..n_knots {
                // println!("Knot {i_knot}");
                let coord_to_follow = coord_knots.get(&(i_knot - 1)).copied().unwrap();
                let mut coord_to_update = coord_knots.entry(i_knot).or_insert((0, 0));
                day09_update_tail(&mut coord_to_update, &coord_to_follow);
                // println! {"New coord for knot {i_knot}: {coord_to_update:?}"};
            }
            let coord_tail: (isize, isize) = coord_knots.get(&last_knot).unwrap().clone();
            let visit_number = visited.entry(coord_tail).or_insert(0);
            *visit_number += 1;
            // println!("The head is at {coord_head:?}, the tail is at {coord_tail:?}");
        }
    }
    let res_part1: usize = visited.len();
    println!("The result for part 2 of day 09 is {res_part1}")
}

fn day10() {
    let file_path: &str = "./inputs/day10/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    //
    let mut register: isize = 1;
    let mut cycle: usize = 1;
    let cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    //
    let mut add_operation: HashMap<usize, isize> = HashMap::new();
    let mut lines_to_be_read: bool = true;
    let mut suspended: bool = false;
    let mut line: &str = match lines.next() {
        Some(l) => l,
        None => panic!("No line in the file, weird"),
    };
    let mut result_part1: isize = 0;
    let mut result_part2: String = String::from("");
    //
    while lines_to_be_read | suspended {
        //println!("At the start of cycle {cycle:>3}, X={register:>3}");
        if cycles.contains(&cycle) {
            result_part1 = result_part1 + cycle as isize * register;
        }
        let cycle2 = (cycle - 1) % 40;
        if (cycle2 as isize >= register - 1) & (cycle2 as isize <= register + 1) {
            result_part2.push_str("#");
        } else {
            result_part2.push_str(".");
        }
        if suspended {
            suspended = false;
        } else {
            if &line[..4] == "noop" {
            } else {
                let value: isize = line[5..].parse().expect("Could not convert to integer");
                add_operation.insert(cycle + 2, value);
                suspended = true;
            }
            line = match lines.next() {
                Some(l) => l,
                None => {
                    lines_to_be_read = false;
                    &"0"
                }
            };
        }
        // println!("hashmap is {:?}", { add_operation.clone() });
        cycle = cycle + 1;
        let possible_addx = add_operation.remove(&cycle).unwrap_or(0);
        register = register + possible_addx;
    }
    println!("The result for part 1 of day 10 is {result_part1}");
    println!("The result for part 2 of day 10 is:");
    for chunk in &result_part2.chars().chunks(40) {
        for c in chunk {
            print!("{}", c)
        }
        println!();
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    add: usize,
    mult: usize,
    square: usize,
    divider: usize,
    destination_true: u8,
    destination_false: u8,
}

impl Default for Monkey {
    fn default() -> Monkey {
        Monkey {
            items: Vec::new(),
            add: 0,
            mult: 0,
            square: 0,
            divider: 0,
            destination_true: 0,
            destination_false: 0,
        }
    }
}

impl Monkey {
    fn new_worry(&self, item: usize) -> usize {
        // Apply operation
        let worry: usize;
        if self.square == 1 {
            worry = item * item;
        } else if self.mult > 0 {
            worry = item * self.mult;
        } else if self.add > 0 {
            worry = item + self.add;
        } else {
            unreachable!("No operation to apply?")
        }
        // Apply relief
        worry / 3
    }
    fn throw_one_item(&self, item: usize) -> (u8, usize) {
        let worry_relief = self.new_worry(item);
        if worry_relief % self.divider == 0 {
            (self.destination_true, worry_relief)
        } else {
            (self.destination_false, worry_relief)
        }
    }
    fn throw_all_items(&self) -> Vec<(u8, usize)> {
        let mut thrown_items: Vec<(u8, usize)> = Vec::new();
        for item in self.items.clone().into_iter() {
            let item_thrown: (u8, usize) = self.throw_one_item(item);
            thrown_items.push(item_thrown)
        }
        thrown_items
    }
}

fn day11() {
    let file_path: &str = "./inputs/day11/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    //
    let mut monkeys: HashMap<u8, Monkey> = HashMap::new();
    // Parse the input file to initialize all monkeys
    //
    let re_monkey_id = match Regex::new("^Monkey ([0-9]+):$") {
        Ok(result) => result,
        Err(error) => {
            panic!("Problem constructing the monkey_id regex {}", error);
        }
    };
    //
    let mut parsing_finished: bool = false;
    let mut line: &str = lines.next().unwrap();
    //
    while !parsing_finished {
        if line.len() > 8 {
            if &line[0..6] == "Monkey" {
                let monkey_id: u8 = match re_monkey_id.captures(line) {
                    Some(match_results) => match_results[1].parse().unwrap(),
                    None => panic!("I did not find the id of the monkey"),
                };
                // parse items
                line = lines.next().unwrap();
                let mut items: Vec<usize> = Vec::new();
                for item_str in line[18..].split(",") {
                    let item_int: usize = item_str.trim().parse().expect("Could not convert");
                    items.push(item_int);
                }
                // Parse operation
                line = lines.next().unwrap();
                let line_short = &line[23..];
                let add: usize;
                let mult: usize;
                let square: usize;
                if line_short == "* old" {
                    add = 0;
                    mult = 0;
                    square = 1;
                } else if &line_short[0..1] == "+" {
                    add = line_short[2..]
                        .parse()
                        .expect("Could not convert add to int");
                    mult = 0;
                    square = 0;
                } else if &line_short[0..1] == "*" {
                    add = 0;
                    mult = line_short[2..]
                        .parse()
                        .expect("Could not convert mult to int");
                    square = 0;
                } else {
                    unreachable!("Could not find the operation.")
                }
                // Parse divider
                line = lines.next().unwrap();
                let divider: usize = line[21..].parse().expect("Could not convert");
                // Parse true
                line = lines.next().unwrap();
                let destination_true: u8 = line[29..].parse().expect("Could not convert");
                // Parse false
                line = lines.next().unwrap();
                let destination_false: u8 = line[30..].parse().expect("Could not convert");
                //
                // Instantiate a new monkey
                let monkey = Monkey {
                    items,
                    add,
                    mult,
                    square,
                    divider,
                    destination_true,
                    destination_false,
                };
                monkeys.insert(monkey_id, monkey);
            }
        }

        line = match lines.next() {
            Some(l) => l,
            None => {
                parsing_finished = true;
                &"0"
            }
        }
    }
    let n_monkeys: u8 = monkeys.len() as u8;
    let mut handled_item_count: HashMap<u8, usize> = HashMap::new();

    // Apply rounds
    let n_rounds: u8 = 20;
    for _i_round in 0..n_rounds {
        // println!("Round {_i_round}");
        for i_monkey in 0..n_monkeys {
            let monkey = monkeys.entry(i_monkey).or_insert(Monkey {
                ..Default::default()
            });
            let item_count = handled_item_count.entry(i_monkey).or_insert(0);
            *item_count = *item_count + monkey.items.len();
            //
            let thrown_items: Vec<(u8, usize)> = monkey.throw_all_items();
            monkey.items.clear();
            //

            for (destination_monkey, item) in thrown_items {
                let monkey = monkeys.entry(destination_monkey).or_insert(Monkey {
                    ..Default::default()
                });
                monkey.items.push(item)
            }
        }
        // for i_monkey in 0..n_monkeys {
        //    let monkey = monkeys.get(&i_monkey).unwrap();
        //    println!("Monkey {i_monkey} is {:?}", monkey);
        //}
    }
    // println!("{:?}", handled_item_count);
    let mut total_handled: Vec<usize> = handled_item_count.values().cloned().collect();
    total_handled.sort_by(|a, b| b.cmp(a));
    // println!("{}, {}", total_handled[0], total_handled[1]);
    let res_part1: usize = total_handled[0] * total_handled[1];
    println!("The result for part 1 of day 11 is {res_part1}");
}

fn main() {
    println!("Hello, world!");
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
}
