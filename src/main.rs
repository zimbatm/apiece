extern crate rustc_serialize;
extern crate docopt;

mod bases;
mod bootstrap;
mod cli;

fn main() {
  let args = cli::get_args();

  if args.cmd_new {
    bootstrap::run(args);
  }
}
