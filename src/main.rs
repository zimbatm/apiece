extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

static USAGE: &'static str = "apiece

Usage:
  apiece new [-d DIR] <base> <package>
  apiece build [local|dev] [-d DIR]
  apiece run [local|dev] [-d DIR -p <port> --forward-ssh-agent --dockeropt=OPT...]
  apiece exec [local|dev] [-d DIR --forward-ssh-agent --dockeropt=OPT...] [--] <command>...
  apiece clean (local|dev) [-d DIR --dockeropt=OPT...]
  apiece info name [-d DIR]
  apiece info base [-d DIR]
  apiece bases [<pattern>]

Options:
  -h --help                 Show this screen.
  -v --version              Show version.
  -d DIR --directory DIR    Service root directory.
  -p PORT --port DIR        Expose service on given port.";

#[derive(RustcDecodable, Debug)]
struct Args {
  cmd_local: bool,
  cmd_dev: bool,

  cmd_new: bool,
  cmd_build: bool,
  cmd_run: bool,
  cmd_exec: bool,
  cmd_clean: bool,

  cmd_info: bool,
  cmd_name: bool,
  cmd_base: bool,

  cmd_bases: bool,

  arg_base: String,
  arg_package: String,
  arg_command: Vec<String>,
  arg_pattern: String,

  flag_directory: String,
  flag_port: Option<u16>,
  flag_forward_ssh_agent: bool,
  flag_dockeropt: Vec<String>,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

  println!("{:?}", args);
}
