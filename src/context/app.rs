use std::ffi::OsString;

use context::EnvVars;

pub struct App {
  name: String,
  workdir: OsString,
  env: EnvVars,
}

impl<'a> App {
  pub fn new(name: String, workdir: OsString, env: EnvVars) -> App {
    App { name: name, workdir: workdir, env: env }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn workdir(&'a self) -> &'a OsString {
    &self.workdir
  }

  pub fn env_vars(&'a self) -> EnvVars {
    let mut env = self.env.clone();
    env.insert("APIECEIO_NAME".to_string(), self.name.clone());
    env
  }
}
