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

pub fn in_docker_context<S: AsRef<OsStr>>(context: &docker::Context, image: &str, temp: bool, program: S, args: &[S]) -> Command {
  let mut command = in_host_context(context, "docker", &vec!["run"]);
  command
    .arg("-w")
    .arg(context.container_dir())
    .arg("-t")
    .arg("-i");

  if temp {
    command.arg("--rm");
  }

  for (k, v) in context.container_env() {
    command.arg("-e").arg(format!("{}={}", k, v));
  }

  match context.external_port() {
    Some(port) => {
      command
        .arg("-p")
        .arg(format!("{}:{}", port, context.container_bind_port()));
    }
    None => {}
  }
  if context.mount_workdir() {
    let host_dir = context.host_dir().to_os_string().into_string().unwrap();
    let container_dir = context.container_dir().to_os_string().into_string().unwrap();
    command
      .arg("-v")
      .arg(format!("{}:{}", host_dir, container_dir));
  }
  match context.ssh_auth_sock() {
    Some(sock) => {
      let host_sock = sock.clone().into_string().unwrap();
      let guest_sock = "/apiece.io/.ssh_auth_sock";
      command
        .arg("-v")
        .arg(format!("{}:{}", host_sock, guest_sock))
        .arg("-e")
        .arg(format!("{}={}", "SSH_AUTH_SOCK", guest_sock));
    }
    None => {}
  }

  command
    .arg("--name")
    .arg(context.container_name())
    .arg(image)
    .arg(program)
    .args(args);

  command
}
