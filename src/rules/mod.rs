use crate::constants::{GenerateRule, Rule};

pub mod bf;
pub mod lf;
pub mod nf;
pub mod ff;

pub fn get_value_by_number(number: u8, rule: GenerateRule) -> Result<Rule, String> {
    match number {
        0 => Ok(rule.zero),
        1 => Ok(rule.one),
        2 => Ok(rule.two),
        3 => Ok(rule.three),
        4 => Ok(rule.four),
        5 => Ok(rule.five),
        6 => Ok(rule.six),
        7 => Ok(rule.seven),
        8 => Ok(rule.eight),
        9 => Ok(rule.nine),
        _ => Err("unexpected number to get value".to_string()),
    }
}
