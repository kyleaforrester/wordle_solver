use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

enum Rule {
    Right(usize, char),
    Wrong(usize, char),
    No(Vec<char>),
}

impl Rule {
    fn new(string: &str) -> Rule {
        if string.contains(" at ") {
            let tup: Vec<&str> = string.split(" at ").collect();
            Rule::Right(
                tup[1].parse::<usize>().unwrap(),
                tup[0].chars().nth(0).unwrap(),
            )
        } else if string.contains(" not ") {
            let tup: Vec<&str> = string.split(" not ").collect();
            Rule::Wrong(
                tup[1].parse::<usize>().unwrap(),
                tup[0].chars().nth(0).unwrap(),
            )
        } else if string.starts_with("no ") {
            let csv = string.strip_prefix("no ").unwrap();
            Rule::No(
                csv.split(',')
                    .map(|x| x.chars().nth(0).unwrap())
                    .collect::<Vec<char>>(),
            )
        } else {
            panic!("Unknown rule: {}", string);
        }
    }

    fn is_valid(&self, string: &str) -> bool {
        match self {
            Rule::Right(u, c) => string.chars().nth(*u).unwrap() == *c,
            Rule::Wrong(u, c) => string.chars().nth(*u).unwrap() != *c && string.contains(*c),
            Rule::No(v) => !string.chars().any(|x| v.contains(&x)),
        }
    }
}

fn rules_filter(string: &str, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|x| x.is_valid(string))
}

fn is_valid_word(string: &str) -> bool {
    string.len() == 5 && string.chars().all(|x| x.is_ascii_lowercase())
}

fn score_word(word: &str, char_scores: &HashMap<char, u32>, rules: &Vec<Rule>) -> u32 {
    let mut skip_idx = Vec::new();

    for r in rules.iter() {
        match r {
            Rule::Right(u, _c) => skip_idx.push(*u),
            _ => (),
        }
    }

    let chars: Vec<char> = word.chars().collect();
    let mut unsolved_chars = Vec::new();

    for i in 0..chars.len() {
        if !skip_idx.contains(&i) {
            unsolved_chars.push(chars[i]);
        }
    }

    unsolved_chars.sort();
    unsolved_chars.dedup();

    unsolved_chars.iter().map(|x| char_scores[&x]).sum()
}

fn main() {
    let usage = "Usage: ./wordle_solver dictionary.txt rules.txt";

    let mut dictionary = File::open(env::args().nth(1).expect(usage)).unwrap();
    let mut rules = File::open(env::args().nth(2).expect(usage)).unwrap();

    let mut buffer = String::new();
    dictionary.read_to_string(&mut buffer).unwrap();

    let mut words: Vec<String> = buffer
        .split('\n')
        .map(|x| x.trim().to_string())
        .filter(|x| is_valid_word(x))
        .collect();

    buffer = String::new();
    rules.read_to_string(&mut buffer).unwrap();

    let rules: Vec<Rule> = buffer
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| Rule::new(x))
        .collect();

    words = words
        .iter()
        .filter(|x| rules_filter(x, &rules))
        .map(|x| x.to_string())
        .collect();

    let mut char_scores: HashMap<char, u32> = HashMap::new();

    for w in words.iter() {
        for c in w.chars() {
            let score = char_scores.entry(c).or_insert(0);
            *score += 1;
        }
    }
    words.sort_by_key(|x| score_word(x, &char_scores, &rules));
    words.reverse();

    println!("{}", words[0]);

    println!("\nOther good options:");
    for w in words.iter().skip(1).take(5) {
        println!("{}", w);
    }
}
