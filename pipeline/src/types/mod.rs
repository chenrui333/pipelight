// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

#![allow(dead_code)]

// pub mod list;
mod traits;

use exec::types::StrOutput;
use exec::Exec;
use log::{debug, error, info, trace, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
// use std::cmp::PartialEq;
use std::error::Error;
use std::fs;
use std::process;

// Global Vars
use once_cell::sync::Lazy;
use std::sync::Mutex;

use utils;
use utils::git::{Git, Hook};
use utils::logger::Logger;
use uuid::Uuid;

const CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Clone)]
pub struct Config {
    pipelines: Option<Vec<Pipeline>>,
    hooks: Option<Vec<Hook>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub pid: Option<u32>,
    pub name: String,
    pub date: Option<String>,
    pub status: Option<Status>,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<Step>,
}
impl Pipeline {
    pub fn log(&self) {
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    pub fn is_running(&mut self) -> bool {
        let res = Logs::get();
        match res {
            Ok(pipelines) => {
                let pipeline = pipelines
                    .iter()
                    .filter(|p| p.name == self.name)
                    .cloned()
                    .next();
                if pipeline.is_none() {
                    return false;
                }
                let is = pipeline.unwrap().pid.is_some();
                return is;
            }
            Err(_) => return false,
        };
    }
    pub fn run(&mut self) {
        if self.is_running() {
            return;
        }
        const PIPELINE: Lazy<Mutex<Pipeline>> = Lazy::new(|| Mutex::new(Pipeline::new()));

        let pid = process::id();
        self.pid = Some(pid);
        let pipeline: &mut Pipeline = self;
        let pipeline_ptr: *mut Pipeline = pipeline;
        Logger::file(&pipeline.uuid);

        unsafe {
            pipeline_ptr.as_mut().unwrap().log();
            pipeline_ptr.as_mut().unwrap().status(&Status::Running);
        }
        for step in &mut self.steps {
            step.run(pipeline_ptr);
        }
        PIPELINE.lock().unwrap().pid = None;
        PIPELINE.lock().unwrap().status(&Status::Succeeded);
        PIPELINE.lock().unwrap().log();
    }
    pub fn status(&mut self, status: &Status) {
        self.status = Some(status.to_owned());
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Step {
    pub name: String,
    pub commands: Vec<Command>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl Step {
    fn run(&mut self, pipeline_ptr: *mut Pipeline) {
        for command in &mut self.commands {
            command.run(pipeline_ptr);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Command {
    pub stdin: String,
    output: Option<StrOutput>,
}
impl Command {
    fn new() -> Command {
        return Command {
            stdin: "".to_owned(),
            output: None,
        };
    }
    fn run(&mut self, pipeline_ptr: *mut Pipeline) {
        let output_res = Exec::new().simple(&self.stdin.clone());
        match output_res {
            Ok(output) => {
                self.output = Some(output);
                Ok(())
            }
            Err(e) => {
                unsafe {
                    pipeline_ptr.as_mut().unwrap().status(&Status::Failed);
                }
                Err(e)
            }
        };
        unsafe {
            pipeline_ptr.as_mut().unwrap().log();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Trigger {
    pub action: Option<Hook>,
    pub branch: Option<String>,
}
impl Trigger {
    /// Return actual triggering env
    pub fn env() -> Result<Trigger, Box<dyn Error>> {
        let branch = Git::new().get_branch()?;
        let action = Hook::origin()?;
        Ok(Trigger {
            branch: Some(branch),
            action: Some(action),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Logs;

impl Logs {
    /// Return pipelines from log files
    pub fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        // let dir = Logger::get().directory;
        let dir = Logger::new().directory;
        let paths = fs::read_dir(dir).unwrap();
        let mut pipelines = vec![];
        for res in paths {
            let dir_entry = res?;
            let json = utils::read_last_line(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<Pipeline>(&json)?;
            pipelines.push(pipeline);
        }
        Ok(pipelines)
    }
}
