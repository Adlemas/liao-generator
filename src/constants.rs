use rand::{thread_rng, Rng};

use crate::generator::generate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Increment,
    Decrement,
}

impl Operation {
    pub fn opposite(&self) -> Operation {
        match self {
            Operation::Increment => Operation::Decrement,
            Operation::Decrement => Operation::Increment,
        }
    }

    pub fn get_random() -> Operation {
        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            Operation::Increment
        } else {
            Operation::Decrement
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GenerateFormula {
    NF,
    LF,
    BF,
    FF,
}

#[derive(Debug)]
pub struct GenerateOptions {
    pub formula: GenerateFormula,
    pub min: i64,
    pub max: i64,
    pub len: usize,
}

impl GenerateOptions {
    // For tests
    #[allow(dead_code)]
    pub fn with_formula(&mut self, formula: GenerateFormula) {
        self.formula = formula;
    }

    pub fn generate(&self) -> Vec<i64> {
        generate(&self)
    }
}

pub type Rule = Vec<u8>;

#[derive(Debug, Clone)]
pub struct GenerateRule {
    pub zero: Rule,
    pub one: Rule,
    pub two: Rule,
    pub three: Rule,
    pub four: Rule,
    pub five: Rule,
    pub six: Rule,
    pub seven: Rule,
    pub eight: Rule,
    pub nine: Rule,
}

#[derive(Debug)]
pub struct GenerateRules {
    pub increment: GenerateRule,
    pub decrement: GenerateRule,
}

// impl GenerateRules {
//     pub fn get_random_rule(&self) -> GenerateRule {
//         if thread_rng().gen_bool(0.5) {
//             self.increment.clone()
//         } else {
//             self.decrement.clone()
//         }
//     }
// }

pub struct TermOptions {
    pub cur_num: Vec<i64>,
    pub formula: GenerateFormula,
    pub terms_len: i64,
    pub schema: TermSchema,
    pub max: i64,
    pub min: i64,
    pub previos_term: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct FactorOptions {
    pub number: u8,
    pub formula: GenerateFormula,
    pub schema: TermSchema,
    pub forbidden_number: Option<u8>,
}

pub struct SchemaOptions {
    pub terms_len: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct TermSchema {
    /**
     * If true, the factor can be zero.
     */
    pub can_generate_zero: bool,
    /**
     * If true, the factor will be generated with
     * first rule array items (before "0" item).
     */
    pub force_formula: bool,
    /**
     * Operation for the factor.
     */
    pub operation: Option<Operation>,
}
