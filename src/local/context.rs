use std::ffi::OsString;
use std::path::PathBuf;

use context::*;

pub struct Context {
  pub app_env: AppEnvironment,
  pub bind: String,
  pub port: u16,
}

impl AppContext for Context {
  fn app_env(&self) -> &AppEnvironment {
    &self.app_env
  }
}

impl HostContext for Context {
  fn host_env(&self) -> EnvVars {
    let mut env_vars = self.app_env.env_vars();
    env_vars.insert("APIECEIO_BIND".to_string(), self.bind.to_string());
    env_vars.insert("APIECEIO_PORT".to_string(), self.port.to_string());
    env_vars.insert("APIECEIO_DATA_DIR".to_string(), self.host_data_dir().into_string().unwrap());
    env_vars
  }
}

impl Context {
  pub fn build_script(&self) -> OsString {
    self.host_script(&self.app_env.build_script())
  }

  pub fn exec_script(&self) -> OsString {
    self.host_script(&self.app_env.exec_script())
  }

  pub fn run_script(&self) -> OsString {
    self.host_script(&self.app_env.run_script())
  }

  pub fn clean_script(&self) -> OsString {
    self.host_script(&self.app_env.clean_script())
  }

  fn host_script(&self, script: &str) -> OsString {
    let mut path = PathBuf::from(self.host_dir());
    path.push(script);
    path.into_os_string()
  }
}
