use std::ffi::OsStr;
use std::process::Command;

use context::HostContext;

pub fn in_host_context<S: AsRef<OsStr>>(context: &HostContext, program: S, args: &[S]) -> Command {
  let mut command = Command::new(program);
  command.args(args);
  for (k, v) in context.host_env() {
    command.env(k, v);
  }
  command.current_dir(context.host_dir());
  command
}
