use super::{Config, Pipeline};
use crate::types;
use exec::Exec;
use log::{debug, error, info, trace, warn};
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use utils::log::Logs;

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        // Git::new();
        let file = "pipelight.config.mjs".to_owned();
        let mut config = Config {
            file: file,
            logs: None,
            triggers: None,
            pipelines: None,
        };
        if config.exists() {
            config = Config::load()?;
            config = Config::check_duplicates(&config)?;
            config.triggers()?;
        }
        Ok(config)
    }
    pub fn logs(&mut self) {
        self.logs = Some(Logs::new());
    }
    pub fn triggers(&self) -> Result<Vec<types::Trigger>, Box<dyn Error>> {
        let mut triggers = vec![];
        if self.pipelines.is_some() {
            let pipelines = self
                .clone()
                .pipelines
                .unwrap()
                .iter()
                .map(|p| types::Pipeline::from(p))
                .collect::<Vec<types::Pipeline>>();

            triggers = pipelines
                .iter()
                .map(|p| p.clone().triggers.unwrap())
                .collect::<Vec<Vec<types::Trigger>>>()
                .into_iter()
                .flatten()
                .collect::<Vec<types::Trigger>>();
        }
        Ok(triggers)
    }
    pub fn pipeline(&self, name: &str) -> Result<Pipeline, Box<dyn Error>> {
        let pipeline_result = self
            .clone()
            .pipelines
            .unwrap()
            .iter()
            .filter(|p| p.name == name)
            .cloned()
            .next();
        match pipeline_result {
            Some(res) => return Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline {:?}", name);
                warn!("{}", message);
                return Err(Box::from(message));
            }
        };
    }
    /// Ensure config file exists
    fn exists(&self) -> bool {
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), self.file);
        let path = Path::new(&string);
        let exist = Path::new(path).exists();
        if !exist {
            let message = "Config file not found.";
            let hint =
                "Use \"pipelight init\" to generate config file\n or move to the right directory";
            error!("{}", message);
            debug!("{}", hint);
        }
        return exist;
    }

    /// Return the config from .mjs file inside the working dir.
    fn load() -> Result<Config, Box<dyn Error>> {
        let executable = "node -e";
        let script = r#"'
            const stock = console;
            console = {};
            const promess = import(`./pipelight.config.mjs`);
            promess
              .then((res) => {
                console = stock;
                const config = res.default;
                const json = JSON.stringify(config, null, 2);
                console.log(json);
              })
              .catch((err) => {
                console.log(err);
              });
        '"#;
        let command = format!("{} {}", executable, script);
        let data = Exec::new().attached(&command)?;
        let config_result = serde_json::from_str::<Config>(&data);
        match config_result {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                let message = format!("From config file: {}", e);
                warn!("{}", message);
                debug!("Json output:\n{}", data);
                return Err(Box::from(message));
            }
        };
    }
    fn check_duplicates(config: &Config) -> Result<Self, Box<dyn Error>> {
        let names = config
            .clone()
            .pipelines
            .unwrap()
            .iter()
            .map(|p| &p.name)
            .cloned()
            .collect::<Vec<String>>();
        //Clone vector and remove duplicates
        let mut dedup = names.clone();
        dedup.sort();
        dedup.dedup();
        let has_duplicate = dedup.len() != names.len();
        if has_duplicate {
            let message = "Duplicate pipeline names in config";
            warn!("{}", message);
            Err(Box::from(message))
        } else {
            Ok(config.to_owned())
        }
    }
    fn clean_hooks(config: &Config) -> Result<Self, Box<dyn Error>> {
        let names = config
            .clone()
            .pipelines
            .unwrap()
            .iter()
            .map(|p| &p.name)
            .cloned()
            .collect::<Vec<String>>();
        //Clone vector and remove duplicates
        let mut dedup = names.clone();
        dedup.sort();
        dedup.dedup();
        let has_duplicate = dedup.len() != names.len();
        if has_duplicate {
            let message = "Duplicate pipeline names in config";
            warn!("{}", message);
            Err(Box::from(message))
        } else {
            Ok(config.to_owned())
        }
    }
}
