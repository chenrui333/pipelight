use super::run;
use crate::cli::utils::detach::detach;
use crate::workflow::{Config, Trigger};
use log::{debug, trace};
use utils::git::Flag;

// Error Handling
use miette::Result;

/// Function to be called from the cli.
/// Either spawn detached new processes or spawn attached threads
/// to run the triggerable pipelines
pub fn launch(attach: bool, flag: Option<String>) -> Result<()> {
    trace!("Create detached subprocess");
    // Run or Fork
    match attach {
        true => {
            trigger(attach, flag)?;
        }
        false => detach(None)?,
    }
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn trigger(attach: bool, flag: Option<String>) -> Result<()> {
    let config = Config::get()?;

    // Set triggering env action
    Trigger::default();
    if flag.is_some() {
        Trigger::flag(Some(Flag::from(&flag.clone().unwrap())))?;
    }

    if config.pipelines.is_none() {
        let message = "No pipeline found";
        debug!("{}", message);
        return Ok(());
    }
    for pipeline in &config.pipelines.unwrap() {
        if pipeline.clone().triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
            debug!("{}", message)
        } else if pipeline.is_triggerable()? {
            run::launch(pipeline.clone().name, attach, flag.clone())?;
        }
    }
    Ok(())
}
