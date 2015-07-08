use std::io::Error;

use command_ext::*;
use commands;
use local::Context;

pub use super::build::build;

pub fn run(context: &Context) -> Result<(), Error> {
  commands::in_host_context(
    context, context.exec_script(), &vec![context.run_script()]
  ).exec()
}

pub fn exec(context: &Context, command: &[String]) -> Result<(), Error> {
  commands::in_host_context(
    context,
    command.first().unwrap().clone(),
    command.split_at(1).1
  ).exec()
}
