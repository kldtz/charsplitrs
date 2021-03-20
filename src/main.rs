//! Evaluation script
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;
use std::time::Instant;

use charsplitrs::CharSplitter;

fn main() {
    run_evaluation();
}

fn run_evaluation() {
    let splitter = CharSplitter::new("data/ngram_probs.json")
    .unwrap_or_else(|err| {
        eprintln!("Error while reading probabilities: {}", err);
        process::exit(1);
    });

    let now = Instant::now();
    germanet_evaluation(&splitter);
    println!("Elapsed time: {:.3?}", now.elapsed());
}

pub fn germanet_evaluation(splitter: &CharSplitter) {
    let mut cases: f64 = 0.0;
    let mut correct: f64 = 0.0;

    let file = File::open("data/split_compounds_from_GermaNet15.0.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    // skip header
    lines.next();
    lines.next();

    for line in lines {
        cases += 1.0;
        let line_str = line.unwrap();
        let fields: Vec<&str> = line_str.trim().split("\t").collect();
        // ignore corrupted lines
        if fields.len() != 3 {
            continue;
        }
        let split_result = splitter.split(fields[0]);
        if titlecase(split_result.1) == fields[2] {
            correct += 1.0;
        }
        
        if (cases as i32) % 10000 == 0 {
            print_accuracy(cases, correct);
        }
    }
    print_accuracy(cases, correct);
}

fn print_accuracy(cases: f64, correct: f64) {
    println!(
        "Accuracy ({} / {}): {}",
        correct,
        cases,
        100.0 * correct / cases
    );
}

pub fn titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}