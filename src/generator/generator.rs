use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    constants::*,
    rules::{get_value_by_number, lf::new_lf_rule, nf::new_nf_rule},
};
