use crate::constants::{SchemaOptions, TermSchema, Operation};

pub fn generate_schema(options: &SchemaOptions) -> Vec<TermSchema> {
    let mut schema: Vec<TermSchema> = vec![];

    for i in 0..options.terms_len {
        let factor_schema = TermSchema {
            operation: if i == 0 {
                Some(Operation::Increment)
            } else {
                //
                // We can keep the operation as None,
                // 'cause the operation will be setted randomly
                //
                None
            },
            can_generate_zero: if i == 0 { false } else { true },
            force_formula: i % 2 != 0,
        };
        schema.push(factor_schema);
    }

    schema
}
