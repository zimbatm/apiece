use std::io::Error;
use std::ffi::OsString;

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
    context.exec_script(),
    &command.iter().map(|s| { OsString::from(s) }).collect::<Vec<_>>()
  ).exec()
}

pub fn clean(context: &Context) -> Result<(), Error> {
  commands::in_host_context(
    context, context.exec_script(), &vec![context.clean_script()]
  ).exec()
}
