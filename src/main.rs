use std::{env, fs::OpenOptions};

use std::io::prelude::*;

use constants::*;

mod constants;
mod generator;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() - 1 != 3 {
        panic!("Incorrect arguments length.\n  Usage: liao-g <FORMULA> <MIN>-<MAX> <LEN>");
    }

    let formula_str = args[1].as_str();
    let diapason: Vec<&str> = args[2].split("-").collect();
    let min = diapason[0].to_string().parse::<i64>().unwrap();
    let max = diapason[1].to_string().parse::<i64>().unwrap();
    let len = args[3].to_string().parse::<usize>().unwrap();

    if min >= max {
        panic!("Minimum number can not be less than maximum number!");
    }

    let options = GenerateOptions {
        formula: match formula_str {
            "NF" | "NC" => GenerateFormula::NF,
            "LF" => GenerateFormula::LF,
            "BF" => GenerateFormula::BF,
            "FF" => GenerateFormula::FF,
            _ => panic!("Unknown formula: {}", formula_str),
        },
        min,
        max,
        len,
    };

    let terms = options.generate();

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open("terms.txt")
        .unwrap();

    let terms_str: Vec<_> = terms.iter().map(|&x| x.to_string()).collect();

    println!("Generated: {:?}", terms);
    if let Err(e) = writeln!(
        file,
        "{}",
        format!(
            "{:35} \t= {}",
            terms_str.join(" "),
            terms.iter().sum::<i64>()
        )
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[cfg(test)]
mod test;
