use docopt::Docopt;

pub static USAGE: &'static str = "apiece

Usage:
  apiece new [-d DIR] <base> <name>
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
pub struct Args {
  pub cmd_local: bool,
  pub cmd_dev: bool,

  pub cmd_new: bool,
  pub cmd_build: bool,
  pub cmd_run: bool,
  pub cmd_exec: bool,
  pub cmd_clean: bool,

  pub cmd_info: bool,
  pub cmd_name: bool,
  pub cmd_base: bool,

  pub cmd_bases: bool,

  pub arg_base: String,
  pub arg_name: String,
  pub arg_command: Vec<String>,
  pub arg_pattern: String,

  pub flag_directory: Option<String>,
  pub flag_port: Option<u16>,
  pub flag_forward_ssh_agent: bool,
  pub flag_dockeropt: Vec<String>,
}

pub fn get_args() -> Args {
  Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit())
}
