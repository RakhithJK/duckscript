mod exec;
mod exit;
mod process_id;
mod spawn;
mod watchdog;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "process";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(exec::create(&package))?;
    commands.set(exit::create(&package))?;
    commands.set(process_id::create(&package))?;
    commands.set(spawn::create(&package))?;
    commands.set(watchdog::create(&package))?;

    Ok(())
}
