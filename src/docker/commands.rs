use std::io::Error;

use commands;
use command_ext::*;
use docker::Context;

pub use super::build::build;

pub fn run(context: &Context) -> Result<(), Error> {
  commands::in_docker_context(
    context, &context.docker_image(), true, &vec![context.run_script()]
  ).exec()
}

pub fn exec(context: &Context, command: &[String]) -> Result<(), Error> {
  commands::in_docker_context(
    context, &context.docker_image(), true, command
  ).exec()
}
