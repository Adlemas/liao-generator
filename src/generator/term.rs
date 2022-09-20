use super::{factor::generate_factor};

pub fn generate_term(mut options: TermOptions) -> Option<i64> {
    let mut term: Vec<i64> = Vec::new();

    if options.schema.operation.is_none() {
        options.schema.operation = Some(Operation::get_random());
    }

    let mut factor_options = FactorOptions {
        number: 0,
        formula: options.formula,
        schema: options.schema.clone(),
        forbidden_number: None,
    };

    if options.min.to_string().len() == options.max.to_string().len() {
        factor_options.schema.can_generate_zero = false;
    }

    let cur_num = if options.cur_num.clone().len() > options.max.to_string().len() {
        options.cur_num.clone()[options.cur_num.clone().len() - options.max.to_string().len()..]
            .to_vec()
    } else {
        options.cur_num.clone()
    };

    let mut leave_repeating = false;

    for (i, &cn) in cur_num.iter().enumerate() {
        factor_options.number = cn as u8;
        if let Some(f_term) = options.previos_term {
            if !leave_repeating {
                let offset = (f_term.to_string().len() as isize) - cur_num.clone().len() as isize + term.len() as isize;

                let is_simple_formula = match options.formula {
                    GenerateFormula::NF | GenerateFormula::LF => true,
                    _ => false
                };

                if offset < 0 && !is_simple_formula {
                    factor_options.forbidden_number = Some(0);
                    println!("can't generate forbidden\nf_term: {} ; i: {} ; cur_num: {:?}", f_term, i, cur_num);
                } else if offset >= 0 {
                    factor_options.forbidden_number = if i as isize + offset >= 0 {
                        Some(
                            f_term
                            .to_string()
                            .split("")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<u8>().unwrap())
                            .collect::<Vec<u8>>()[offset as usize],
                            )
                    } else {
                        None
                    };
                }        
            }
        }

        if factor_options.forbidden_number.is_some() {
            leave_repeating = true;
        }

        if let Some(f) = generate_factor(&factor_options) {
            term.push(f as i64);

            factor_options.schema = options.schema.clone();
        } else {
            // If we can't generate a term with the current operation,
            // we try to generate a term with a different operation
            // (if the operation is not None).
            if let Some(operation) = options.schema.operation {
                return generate_term(TermOptions {
                    schema: TermSchema {
                        operation: Some(operation.opposite()),
                        ..options.schema.clone()
                    },
                    ..options
                });
            }
        }
    }

    let term_str = term
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    if term_str.parse::<i64>().unwrap() == 0 && options.terms_len > 0 {
        let mut schema_ = options.schema.clone();

        if schema_.operation.is_none() {
            schema_.operation = Some(Operation::get_random());
        } else if let Some(operation) = schema_.operation {
            schema_.operation = Some(operation.opposite());
        }

        return generate_term(TermOptions {
            schema: schema_,
            cur_num: options.cur_num.clone(),
            formula: options.formula,
            terms_len: options.terms_len,
            max: options.max,
            min: options.min,
            previos_term: options.previos_term,
        });
    }

    if let Ok(t) = term_str.parse::<i64>() {
        Some(match options.schema.operation {
            Some(operation) => match operation {
                Operation::Increment => t,
                Operation::Decrement => -t,
            },
            None => panic!("Operation is None"),
        })
    } else {
        None
    }
}
