use self::{schema::generate_schema, term::generate_term};

use crate::constants::{TermOptions, GenerateOptions, SchemaOptions, TermSchema};

mod factor;
mod schema;
mod term;

pub fn generate(options: &GenerateOptions) -> Vec<i64> {
    // check simple rules
    if options.min >= options.max {
        // TODO: handle error here
        panic!("options.min ({}) can't be greater then equal options.max ({})", options.min, options.max);
    }

    let mut terms: Vec<i64> = vec![];
    let mut terms_sum: i64 = 0;

    let formula = &options.formula;

    let schema: Vec<TermSchema> = generate_schema(&SchemaOptions {
        terms_len: options.len,
    });

    let mut cur_num: Vec<i64> = "0"
        .repeat(options.max.to_string().len())
        .split("")
        .map(|x| x.to_string())
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for i in 0..options.len {
        let term_options = TermOptions {
            cur_num: cur_num.clone(),
            formula: *formula,
            terms_len: terms.len() as i64,
            schema: schema.clone()[i],
            max: options.max,
            min: options.min,
            previos_term: if i > 0 {
                Some(terms[i - 1].abs())
            } else {
                None
            },
        };

        let term: Option<i64> = generate_term(term_options);
        if let Some(t) = term {
            terms.push(t);
            terms_sum += t;

            cur_num = terms_sum
                .to_string()
                .replace("-", "")
                .split("")
                .map(|x| x.to_string())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            if options.max.to_string().clone().len() as isize - cur_num.clone().len() as isize > 0 {
                cur_num = "0"
                    .repeat(options.max.to_string().len() - cur_num.len())
                    .split("")
                    .map(|x| x.to_string())
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i64>().unwrap())
                    .chain(cur_num.into_iter())
                    .collect();
            }
        }
    }

    terms
}
