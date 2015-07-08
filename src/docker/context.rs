use std::ffi::OsString;
use std::path::PathBuf;

use regex::Regex;

use context::*;

pub struct Context {
  pub app_env: AppEnvironment,
  pub external_port: Option<u16>,
  pub ssh_auth_sock: Option<OsString>,
  pub docker_options: Vec<String>,
  pub mount_workdir: bool,
}

impl AppContext for Context {
  fn app_env(&self) -> &AppEnvironment {
    &self.app_env
  }
}

impl HostContext for Context {
  fn host_env(&self) -> EnvVars {
    EnvVars::new()
  }
}

impl ContainerContext for Context {
  fn container_bind_port(&self) -> u16 {
    3000
  }

  fn mount_workdir(&self) -> bool {
    self.mount_workdir
  }

  fn container_env(&self) -> EnvVars {
    let mut env_vars = self.app_env.env_vars();
    env_vars.insert(
      "APIECEIO_PORT".to_string(),
      self.container_bind_port().to_string());
    env_vars.insert(
      "APIECEIO_DATA_DIR".to_string(),
      self.container_data_dir().into_string().unwrap());
    env_vars
  }
}

impl Context {
  pub fn external_port(&self) -> Option<u16> {
    self.external_port
  }

  pub fn ssh_auth_sock(&self) -> Option<&OsString> {
    self.ssh_auth_sock.as_ref()
  }

  pub fn build_script(&self) -> OsString {
    self.container_script(&self.app_env.build_script())
  }

  pub fn run_script(&self) -> OsString {
    self.container_script(&self.app_env.run_script())
  }

  pub fn clean_script(&self) -> OsString {
    self.container_script(&self.app_env.clean_script())
  }

  fn container_script(&self, script: &str) -> OsString {
    let mut path = PathBuf::from(self.container_dir());
    path.push(script);
    path.into_os_string()
  }

  pub fn docker_image(&self) -> String {
    let re = Regex::new(r"[^a-z0-9_]").unwrap();
    format!(
      "{}__{}",
      re.replace_all(&self.app_name().replace("/", "__"), "_"),
      self.app_env.name()
    )
  }

  pub fn docker_file(&self) -> OsString {
    let mut docker_file = PathBuf::from(self.host_dir());
    docker_file.push("apiece.io");
    docker_file.push("docker");
    docker_file.push(format!("Dockerfile.{}.x86_64", self.app_env.name()));
    docker_file.into_os_string()
  }

  pub fn container_name(&self) -> String {
    self.docker_image()
  }

  pub fn docker_options(&self) -> &Vec<String> {
    &self.docker_options
  }
}
