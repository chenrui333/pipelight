// Types that exist just to so json_serde can translate json into usable... things
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
pub use utils::log::Logs;
mod config;
use crate::types;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(skip)]
    pub file: String,
    #[serde(skip)]
    pub logs: Option<Logs>,
    #[serde(skip)]
    pub triggers: Option<Vec<types::Trigger>>,
    pub pipelines: Option<Vec<Pipeline>>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<Step>,
    pub triggers: Option<Vec<Trigger>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    pub actions: Option<Vec<String>>,
    pub branches: Vec<String>,
}
