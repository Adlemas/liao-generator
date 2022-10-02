use liao_generator::constants::*;

fn main() {
    let vargs: Vec<String> = std::env::args().collect();
    let mut count: u32 = 1;

    if vargs.len() > 1 {
        count = vargs[1].parse().unwrap();
    }

    let options = GenerateOptions {
        formula: GenerateFormula::FF,
        min: 10,
        max: 99,
        len: 10,
    };

    for _ in 0..count {
        let terms = options.generate();
        println!("{:?}", terms);
    }
}
