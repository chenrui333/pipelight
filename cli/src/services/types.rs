// Struct
use crate::services::traits::Parser;
use crate::types::{Cli, Commands};
// Error Handling
use miette::Result;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Actions {
    Run,
    Trigger,
    Watch,
}

/**
If you want to create a detached or attached running process fork.
If you want to execute actions in the same process,
use Actions directly.
*/
#[derive(Debug, Clone)]
pub struct Service {
    pub args: Option<Cli>,
    pub cmd: Actions,
}

impl Service {
    pub fn new(cmd: Actions, args: Option<Cli>) -> Result<Self> {
        let mut service = Service { cmd, args };
        // println!("{:#?}", service);
        service.convert()?;
        // println!("{:#?}", service);
        Ok(service)
    }
}