use std::ffi::OsString;
use std::io::Error;

use commands;
use command_ext::*;
use docker::Context;

pub use super::build::build;

pub fn run(context: &Context, args: Vec<String>) -> Result<(), Error> {
  let mut command = vec![context.run_script()];
  for arg in args {
    command.push(OsString::from(arg));
  }
  commands::in_docker_context(
    context, &context.docker_image(), true, &command
  ).exec()
}

pub fn exec(context: &Context, command: &[String]) -> Result<(), Error> {
  commands::in_docker_context(
    context, &context.docker_image(), true, command
  ).exec()
}

pub fn clean(context: &Context) -> Result<(), Error> {
  commands::in_docker_context(
    context, &context.docker_image(), true, &vec![context.clean_script()]
  ).exec()
}
