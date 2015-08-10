mod build;
mod context;
mod commands;

pub struct Bind {
  pub address: String,
  pub port: u16,
}

pub enum Network {
  Host(Bind),
  Bridge(Option<Bind>),
}

pub use self::context::Context;
pub use self::commands::{build, run, exec, clean};
