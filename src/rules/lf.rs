use crate::constants::{GenerateRule, GenerateRules};

pub fn new_lf_rule() -> GenerateRules {
    GenerateRules {
        increment: GenerateRule {
            zero: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            one: vec![4, 0, 1, 2, 3, 5, 6, 7, 8],
            two: vec![3, 4, 0, 1, 2, 5, 6, 7],
            three: vec![2, 3, 4, 0, 1, 5, 6],
            four: vec![1, 2, 3, 4, 0, 5],
            five: vec![0, 1, 2, 3, 4],
            six: vec![0, 1, 2, 3],
            seven: vec![0, 1, 2],
            eight: vec![0, 1],
            nine: vec![0],
        },
        decrement: GenerateRule {
            zero: vec![0],
            one: vec![0],
            two: vec![0, 1],
            three: vec![0, 1, 2],
            four: vec![0, 1, 2, 3],
            five: vec![1, 2, 3, 4, 0],
            six: vec![2, 3, 4, 0, 1, 5],
            seven: vec![3, 4, 0, 1, 2, 5, 6],
            eight: vec![4, 0, 1, 2, 3, 5, 6, 7],
            nine: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        },
    }
}
