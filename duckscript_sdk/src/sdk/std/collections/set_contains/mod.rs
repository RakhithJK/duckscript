use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, mutate_set};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SetContains")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_contains".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Set handle not provided.".to_string())
        } else if arguments.len() < 2 {
            CommandResult::Error("Value not provided.".to_string())
        } else {
            let state = get_handles_sub_state(state);

            let key = arguments[0].clone();

            let result = mutate_set(key, state, |set| {
                let item = set.contains(&arguments[1]);
                Ok(Some(item.to_string()))
            });

            match result {
                Ok(value) => CommandResult::Continue(value),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
