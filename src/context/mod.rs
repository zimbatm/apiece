mod app;
mod app_environment;

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

pub type EnvVars = HashMap<String, String>;

pub use self::app::App;
pub use self::app_environment::AppEnvironment;

pub trait AppContext {
  fn app_env(&self) -> &AppEnvironment;
}

pub trait HostContext : AppContext {
  fn host_dir(&self) -> &OsStr {
    self.app_env().workdir()
  }

  fn host_data_dir(&self) -> OsString {
    let mut data_dir = PathBuf::from(self.host_dir());
    data_dir.push(self.app_env().data_dir());
    data_dir.into_os_string()
  }

  fn host_env(&self) -> EnvVars;
}

pub trait ContainerContext : AppContext {
   fn container_dir(&self) -> &OsStr {
    OsStr::new("/apiece.io/checkout/")
  }

  fn container_data_dir(&self) -> OsString {
    let mut data_dir = PathBuf::from(self.container_dir());
    data_dir.push(self.app_env().data_dir());
    data_dir.into_os_string()
  }

  fn container_env(&self) -> EnvVars;
  fn mount_workdir(&self) -> bool;
}
