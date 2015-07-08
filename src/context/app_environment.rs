use std::ffi::{OsString, OsStr};

use context::EnvVars;
use context::App;

pub struct AppEnvironment {
  name: &'static str,
  app: App,
}

impl AppEnvironment {
  pub fn new(name: &'static str, app: App) -> AppEnvironment {
    AppEnvironment { name: name, app: app }
  }

  pub fn name(&self) -> &str {
    self.name
  }

  pub fn app_name(&self) -> &str {
    &self.app.name()
  }

  pub fn workdir(&self) -> &OsStr {
    self.app.workdir()
  }

  pub fn env_vars(&self) -> EnvVars {
    let mut env = self.app.env_vars();
    env.insert("APIECEIO_ENVIRONMENT".to_string(), self.name.to_string());
    env
  }

  pub fn data_dir(&self) -> OsString {
    OsString::from(format!("apiece.io/data/{}", self.name))
  }

  pub fn build_script(&self) -> String {
    self.script("build")
  }

  pub fn exec_script(&self) -> String {
    self.script("exec")
  }

  pub fn run_script(&self) -> String {
    self.script("run")
  }

  fn script(&self, name: &str) -> String {
    format!("apiece.io/{}/{}", name, self.name)
  }
}
