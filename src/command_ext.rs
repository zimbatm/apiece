use std::io::{Error, ErrorKind};
use std::process::Command;

pub trait CommandExec {
  fn exec(&mut self) -> Result<(), Error>;
}

impl CommandExec for Command {
  fn exec(&mut self) -> Result<(), Error> {
    let status = try!(self.status());
    if status.success() {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Other, "Exited with non-zero code"))
    }
  }
}
