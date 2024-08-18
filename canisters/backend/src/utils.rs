use candid::CandidType;
use rand::Rng;
use serde::Deserialize;

use crate::{mutate_rng, random_value};

#[derive(CandidType, Deserialize)]
pub struct CaptchaRequirement {
    pub char_count: u8,
    pub is_alpha_numeric: bool,
}

pub fn generate_text(
    CaptchaRequirement {
        char_count,
        is_alpha_numeric,
    }: CaptchaRequirement,
) -> String {
    if !(1..=10).contains(&char_count) {
        ic_cdk::trap("Length too Long")
    }
    let mut word = String::new();
    if is_alpha_numeric {
        for i in 0..char_count {
            ic_cdk::println!("loop index: {i}");
            let is_numeric_flag = random_value::gen_bool();
            ic_cdk::println!("boolean generated: {is_numeric_flag}");
            if is_numeric_flag {
                let generated_value: u8 = mutate_rng(|rng| rng.gen_range(0..=9));
                ic_cdk::println!("value generated: {generated_value}");
                word = format!("{word}{generated_value}");
            } else {
                let is_capital = random_value::gen_bool();
                let (from, to) = if is_capital { (65, 91) } else { (97, 123) };
                let generated_value: u8 = mutate_rng(|rng| rng.gen_range(from..to));
                ic_cdk::println!("value generated: {generated_value}");
                word.push(generated_value as char);
            }
        }
    } else {
        for i in 0..char_count {
            ic_cdk::println!("loop index: {i}");
            let is_capital = random_value::gen_bool();
            ic_cdk::println!("value generated: {is_capital}");
            let (from, to) = if is_capital { (65, 91) } else { (97, 123) };
            let generated_value: u8 = mutate_rng(|rng| rng.gen_range(from..to));
            ic_cdk::println!("value generated: {generated_value}");
            word.push(generated_value as char);
        }
    }
    word
}
