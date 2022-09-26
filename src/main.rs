use liao_generator::constants::*;

fn main() {
    let options = GenerateOptions {
        formula: GenerateFormula::FF,
        min: 10,
        max: 99,
        len: 10,
    };

    let terms = options.generate();

    println!("{:?}", terms);
}
