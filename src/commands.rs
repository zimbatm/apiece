use std::ffi::OsStr;
use std::process::Command;

use context::{ContainerContext, HostContext};
use docker;

pub fn in_host_context<S: AsRef<OsStr>>(context: &HostContext, program: S, args: &[S]) -> Command {
  let mut command = Command::new(program);
  command.args(args);
  for (k, v) in context.host_env() {
    command.env(k, v);
  }
  command.current_dir(context.host_dir());
  command
}

pub fn in_docker_context<S: AsRef<OsStr>>(context: &docker::Context, image: &str, temp: bool, program: &[S]) -> Command {
  let mut command = in_host_context(context, "docker", &vec!["run"]);

  for opt in context.docker_options() {
    command.arg(opt);
  }

  command
    .arg("-w")
    .arg(context.container_dir())
    .arg("-i")
    .arg("--sig-proxy=true");
  if temp {
    command.arg("--rm");
  }

  for (k, v) in context.container_env() {
    command.arg("-e").arg(format!("{}={}", k, v));
  }

  match context.network() {
    &docker::Network::Bridge(Some(ref bind)) => {
      command
        .arg("-p")
        .arg(format!("{}:{}:{}", bind.address, bind.port, context.container_bind_port()))
        .arg("-e")
        .arg(format!("APIECEIO_BIND={}", "0.0.0.0"));
    }
    &docker::Network::Host(ref bind) => {
      command
        .arg("--net=host")
        .arg("-e")
        .arg(format!("APIECEIO_BIND={}", bind.address));
    }
    _ => {}
  }

  let host_dir = if context.mount_workdir() {
    context.host_dir().to_os_string()
  } else {
    context.host_data_dir()
  }.into_string().unwrap();

  let container_dir = if context.mount_workdir() {
    context.container_dir().to_os_string()
  } else {
    context.container_data_dir()
  }.into_string().unwrap();

  command
    .arg("-v")
    .arg(format!("{}:{}", host_dir, container_dir));

  match context.ssh_auth_sock() {
    Some(sock) => {
      let host_sock = sock.clone().into_string().unwrap();
      let guest_sock = "/apiece.io/.ssh_auth_sock";
      command
        .arg("-v")
        .arg(format!("{}:{}", host_sock, guest_sock))
        .arg("-e")
        .arg(format!("SSH_AUTH_SOCK={}", guest_sock));
    }
    None => {}
  }

  let container_name = match context.instance_name() {
    Some(name) => format!("{}_{}", context.container_name(), name),
    None => context.container_name()
  };

  command
    .arg("--name")
    .arg(container_name)
    .arg(image)
    .arg(context.exec_script())
    .args(program);

  command
}
