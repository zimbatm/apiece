use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Error;
use std::result;

use rand;

use command_ext::*;
use commands;
use context::HostContext;

use docker::Context;

pub type Result<T> = result::Result<T, Error>;

pub fn build(context: &Context) -> Result<()> {
  let tmp_docker_file = try!(make_tmp_docker_file(context));
  let tag_stage1 = try!(build_stage1(context, &tmp_docker_file));
  let tag_stage2 = try!(build_stage2(context, &tag_stage1));
  try!(commit_stage2(context, &tag_stage2));

  try!(remove_stage2(context, tag_stage2));
  try!(remove_stage1(context, tag_stage1));
  fs::remove_file(tmp_docker_file)
}

fn make_tmp_docker_file(context: &Context) -> Result<OsString> {
  let tmp_docker_file = find_unique_filename(&context.host_dir());
  fs::copy(context.docker_file(), &tmp_docker_file)
    .map(|_| { tmp_docker_file })
}

fn build_stage1(context: &Context, tmp_docker_file: &OsStr) -> Result<String> {
  let tag_stage1 = format!("{}__stage1", context.docker_image());
  commands::in_host_context(context, "docker", &vec!["build"])
    .arg("-f").arg(tmp_docker_file)
    .arg("-t").arg(&tag_stage1)
    .arg(".")
    .exec()
    .map(|_| { tag_stage1 })
}

fn remove_stage1(context: &Context, tag_stage1: String) -> Result<()> {
  commands::in_host_context(context, "docker", &vec!["rmi"])
    .arg(tag_stage1)
    .exec()
}

fn build_stage2(context: &Context, tag_stage1: &str) -> Result<String> {
  commands::in_docker_context(
    context, tag_stage1, false, &vec![context.build_script()]
  ).exec().map(|_| { context.container_name() })
}

fn commit_stage2(context: &Context, tag_stage2: &str) -> Result<()> {
  commands::in_host_context(context, "docker", &vec!["commit"])
    .arg(tag_stage2)
    .arg(context.docker_image())
    .exec()
}

fn remove_stage2(context: &Context, tag_stage2: String) -> Result<()> {
  commands::in_host_context(context, "docker", &vec!["rm"])
    .arg(tag_stage2)
    .exec()
}

fn find_unique_filename(prefix: &OsStr) -> OsString {
  loop {
    let mut path = prefix.to_os_string();
    path.push("/Dockerfile.temp.");
    path.push(rand::random::<u16>().to_string());

    match fs::metadata(&path) {
      Err(_) => return path,
      Ok(_) => continue
    }
  }
}
