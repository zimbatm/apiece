use std::io::Error;
use std::process::ExitStatus;

use commands;
use local::Context;

pub fn build(context: &Context) -> Result<ExitStatus, Error> {
  commands::in_host_context(context, context.build_script(), &vec![])
    .status()
}
