extern crate docopt;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;

mod command_ext;

mod bases;
mod bootstrap;
mod cli;
mod commands;

mod context;
mod docker;
mod local;

use std::collections::HashMap;
use std::env;
use std::ffi::{OsString, OsStr};
use std::fs::File;
use std::io::{Read, Error};
use std::path::PathBuf;

use regex::Regex;

use context::AppContext;

fn main() {
  let args = cli::get_args();

  if args.cmd_new {
    bootstrap::run(args);
  } else {
    let workdir = get_workdir(args.flag_directory.as_ref()).unwrap();
    let app_name = read_app_name(&workdir).unwrap();
    let app = context::App::new(app_name, workdir, HashMap::new());

    let bind = args.flag_bind.unwrap_or("127.0.0.1".to_string());

    if args.cmd_local || args.cmd_info {
      let context = local::Context {
        app_env: context::AppEnvironment::new("local", app),
        bind: bind,
        port: args.flag_port.unwrap_or(3000),
      };
      if args.cmd_build {
        local::build(&context).unwrap();
      } else if args.cmd_run {
        local::run(&context, args.arg_args).unwrap();
      } else if args.cmd_exec {
        local::exec(&context, &args.arg_command).unwrap();
      } else if args.cmd_clean {
        local::clean(&context).unwrap();
      } else if args.cmd_info {
        if args.cmd_name {
          println!("{}", context.app_name());
        }
      }
    } else {
      let network = match args.flag_net {
        Some(net_type) => {
          if net_type == "host" {
            match args.flag_port {
              Some(port) =>
                docker::Network::Host(
                  docker::Bind { address: bind, port: port }
                ),
              None => panic!("Host network requires a port number"),
            }
          } else {
            panic!("Unknown network type: {}", net_type)
          }
        },
        None => {
          docker::Network::Bridge(args.flag_port.map(|p| {
            docker::Bind { address: bind, port: p }
          }))
        }
      };

      let context = docker::Context {
        app_env: if args.cmd_dev {
          context::AppEnvironment::new("development", app)
        } else {
          context::AppEnvironment::new("production", app)
        },
        instance_name: args.flag_instance,
        ssh_auth_sock: if args.flag_forward_ssh_agent {
          env::var("SSH_AUTH_SOCK").ok().map(|s| { OsString::from(s) })
        } else {
          None
        },
        docker_options: args.flag_dockeropt,
        mount_workdir: args.cmd_dev,
        network: network,
      };
      if args.cmd_build {
        docker::build(&context).unwrap();
      } else if args.cmd_run {
        docker::run(&context, args.arg_args).unwrap();
      } else if args.cmd_exec {
        docker::exec(&context, &args.arg_command).unwrap();
      } else if args.cmd_clean {
        docker::clean(&context).unwrap();
      }
    };
  }
}

fn get_workdir(directory: Option<&String>) -> Result<OsString, Error> {
  env::current_dir().map(|path| {
    match directory {
      Some(d) => path.join(d),
      None => path
    }.into_os_string()
  })
}

fn read_app_name(workdir: &OsStr) -> Result<String, Error> {
  let mut file = try!(
    File::open(PathBuf::from(workdir).join("apiece.io/app-name"))
  );
  let mut buf = String::new();
  file.read_to_string(&mut buf).map(|_| {
    Regex::new(r"\s*$").unwrap().replace_all(&buf, "")
  })
}
