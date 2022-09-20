use rand::{seq::SliceRandom, thread_rng};

use crate::rules::{bf::new_bf_rule, lf::new_lf_rule, nf::new_nf_rule, *};
use crate::constants::{GenerateRules, GenerateFormula, FactorOptions, Operation};

fn generate_with_rule(rules: GenerateRules, options: &FactorOptions) -> Option<u8> {
    let mut rng = thread_rng();

    let rule = match options.schema.operation {
        Some(op) => match op {
            Operation::Decrement => rules.decrement,
            Operation::Increment => rules.increment,
        },
        None => panic!("Operation is not setted"),
    };

    if let Ok(numbers) = get_value_by_number(options.number, rule) {
        let mut numbers = numbers.clone();

        if let Some(forbidden_number) = options.forbidden_number {
            numbers.retain(|&x| x != forbidden_number);
        }

        let force_formula = options.schema.force_formula;

        if force_formula {
            let force_numbers_stop = numbers.iter().position(|x| *x == 0);
            let mut force_numbers: Vec<u8> = numbers
                .clone()
                .drain(0..force_numbers_stop.unwrap_or(0))
                .collect();

            force_numbers.retain(|x| *x != 0);

            if force_numbers.len() > 0 {
                let force_number = force_numbers.choose(&mut rng).unwrap();
                return Some(*force_number);
            }
        }

        let can_generate_zero = options.schema.can_generate_zero;

        if !can_generate_zero {
            numbers.retain(|&x| x != 0);
        }

        if numbers.is_empty() {
            return None;
        }

        numbers.shuffle(&mut rng);

        return Some(numbers[0]);
    } else {
        None
    }
}

pub fn generate_factor(options: &FactorOptions) -> Option<u8> {
    let rules: Option<GenerateRules> = match options.formula {
        GenerateFormula::NF => Some(new_nf_rule()),
        GenerateFormula::LF => Some(new_lf_rule()),
        GenerateFormula::BF => Some(new_bf_rule()),
        _ => None,
    };

    if let Some(rules) = rules {
        return generate_with_rule(rules, &options);
    }

    None
}
