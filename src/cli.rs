use docopt::Docopt;

// Remember to update the README after changing the interface
pub static USAGE: &'static str = "apiece

Usage:
  apiece new [-d DIR] <base> <name>
  apiece build local [-d DIR]
  apiece build [dev] [-d DIR --forward-ssh-agent]
  apiece run local [-d DIR -b BIND -p PORT] [--] [<args>...]
  apiece run [dev] [-d DIR -b BIND -p PORT -i INSTANCE --forward-ssh-agent --net=NETWORK --dockeropt=OPT...] [--] [<args>...]
  apiece exec local [-d DIR] [--] <command>...
  apiece exec [dev] [-d DIR --forward-ssh-agent --dockeropt=OPT...] [--] <command>...
  apiece clean local [-d DIR]
  apiece clean dev [-d DIR --dockeropt=OPT...]
  apiece info name [-d DIR]

Options:
  -h --help                         Show this screen.
  -v --version                      Show version.
  -d DIR --directory DIR            Service root directory.
  -b BIND --bind BIND               Expose service on given address.
  -p PORT --port PORT               Expose service on given port.
  -i INSTANCE --instance INSTANCE   Set an instance name for the container.";

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

  pub arg_base: String,
  pub arg_name: String,
  pub arg_args: Vec<String>,
  pub arg_command: Vec<String>,

  pub flag_directory: Option<String>,
  pub flag_bind: Option<String>,
  pub flag_port: Option<u16>,
  pub flag_instance: Option<String>,
  pub flag_forward_ssh_agent: bool,
  pub flag_net: Option<String>,
  pub flag_dockeropt: Vec<String>,
}

pub fn get_args() -> Args {
  Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit())
}
