use std::io::Error;

use command_ext::*;
use commands;
use local::Context;

pub fn run(context: &Context) -> Result<(), Error> {
  commands::in_host_context(
    context, context.exec_script(), &vec![context.run_script()]
  ).exec()
}
