use crate::cli::interface::types::ColoredOutput;
use convert_case::{Case, Casing};

use log::error;
use std::process::exit;

impl From<&String> for ColoredOutput {
    fn from(option: &String) -> ColoredOutput {
        let cased: &str = &option.to_case(Case::Kebab);
        serde_plain::from_str(cased).unwrap()
    }
}
impl From<&ColoredOutput> for String {
    fn from(option: &ColoredOutput) -> String {
        serde_plain::to_string::<ColoredOutput>(option).unwrap()
    }
}
