use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use liao_generator::constants::*;

const LOGS_DIR: &str = "logs";

const LOOP_COUNT: usize = 1000;

const TERMS_LEN: usize = 10;
const MIN: i64 = 10;
const MAX: i64 = 99;

const GENERATE_OPTIONS: GenerateOptions = GenerateOptions {
    formula: GenerateFormula::NF,
    min: MIN,
    max: MAX,
    len: TERMS_LEN,
};

pub fn get_formulas() -> Vec<GenerateFormula> {
    vec![
        GenerateFormula::FF,
        GenerateFormula::BF,
        GenerateFormula::NF,
        GenerateFormula::LF,
    ]
}

pub fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn get_log_file(filename: &str) -> File {
    let path = Path::new(LOGS_DIR).join(format!("{}.log", filename));

    if !path.exists() {
        let _ = fs::create_dir_all(LOGS_DIR);
    }

    if path.exists() {
        fs::remove_file(&path).unwrap();
    }

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap()
}

#[test]
pub fn test_terms_length() {
    init();

    let mut options = GENERATE_OPTIONS;

    let mut file = get_log_file("test_terms_length");

    for formula in get_formulas() {
        options.formula = formula;
        for _ in 0..LOOP_COUNT {
            let terms = options.generate();

            file.write(
                format!(
                    "{:?} => {} \t= {}\n",
                    formula,
                    terms
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    terms.iter().sum::<i64>()
                )
                .as_bytes(),
            )
            .unwrap();

            assert_eq!(terms.len(), TERMS_LEN);
        }
    }
}

#[test]
pub fn test_terms_sum() {
    init();

    let mut options = GENERATE_OPTIONS;

    let mut file = get_log_file("test_terms_sum");

    for formula in get_formulas() {
        options.formula = formula;
        for _ in 0..LOOP_COUNT {
            let terms = options.generate();

            file.write(
                format!(
                    "{:?} => {} \t= {}\n",
                    formula,
                    terms
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    terms.iter().sum::<i64>()
                )
                .as_bytes(),
            )
            .unwrap();

            assert!(
                terms.iter().sum::<i64>() > 0,
                "{}",
                format!("{:?} <= 0", terms)
            );
        }
    }
}

#[test]
pub fn test_terms_range() {
    init();

    let mut options = GENERATE_OPTIONS;

    let mut file = get_log_file("test_terms_range");

    for formula in get_formulas() {
        options.formula = formula;
        for _ in 0..LOOP_COUNT {
            let terms = options.generate();

            file.write(
                format!(
                    "{:?} => {} \t= {}\n",
                    formula,
                    terms
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    terms.iter().sum::<i64>()
                )
                .as_bytes(),
            )
            .unwrap();

            assert!(
                terms
                    .iter()
                    .all(|&x| x.abs() >= options.min && x.abs() <= options.max),
                "{}",
                format!("{}", "range failed")
            );
        }
    }
}

#[test]
pub fn test_terms_repeating() {
    init();

    let mut options = GENERATE_OPTIONS;

    let mut file = get_log_file("test_terms_repeating");

    for formula in get_formulas() {
        options.formula = formula;
        for _ in 0..LOOP_COUNT {
            let terms = options.generate();

            file.write(
                format!(
                    "{:?} => {} \t= {}\n",
                    formula,
                    terms
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    terms.iter().sum::<i64>()
                )
                .as_bytes(),
            )
            .unwrap();

            let mut repeated_at: isize = -1;

            for (i, term) in terms.iter().enumerate() {
                if i > 0 && terms[i - 1] == *term {
                    repeated_at = (i - 1) as isize;
                    break;
                }
            }

            assert!(
                repeated_at == -1,
                "{:?} repeated_at: {}",
                terms,
                repeated_at
            );
        }
    }
}
