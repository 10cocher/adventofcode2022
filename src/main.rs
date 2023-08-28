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
    let data_part_1: usize = contents
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

    let data_part_2: usize = contents
        .lines()
        .map(|line| {
            let mut toto = line.chars();
            let a = toto.nth(0).unwrap();
            let b = toto.nth(1).unwrap();
            let rps = parse_input(b, a);
            println!("{}, {}, {:?}, {}", a, b, rps, rps.score_total());
            rps.score_total()
        })
        .sum();
    println!("{:?} {:?}", &data_part_1, &data_part_2);
    // let row = contents[0]
    //for number in 1..4 {
    //    println!("{number}!");
    //}

    //println!("Content of the file:\n{row}");
}

fn main() {
    println!("Hello, world!");
    day02()
}
