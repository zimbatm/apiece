extern crate nix;

use std::io::{Error, ErrorKind};
use std::process::Command;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

use self::nix::sys::signal;

static INTERRUPT: AtomicBool = ATOMIC_BOOL_INIT;
static TERMINATE: AtomicBool = ATOMIC_BOOL_INIT;

extern fn handle_sigint(_:i32) {
  INTERRUPT.store(true, SeqCst);
}

extern fn handle_sigterm(_:i32) {
  TERMINATE.store(true, SeqCst);
}

pub trait CommandExec {
  fn exec(&mut self) -> Result<(), Error>;
}

impl CommandExec for Command {
  fn exec(&mut self) -> Result<(), Error> {
    let mut child = try!(self.spawn());
    let id = child.id() as i32;

    thread::spawn(move || {
      let mut sigint = false;
      let mut sigterm = false;

      while !sigint && !sigterm {
        sigint = INTERRUPT.load(SeqCst);
        sigterm = TERMINATE.load(SeqCst);
        thread::sleep_ms(500);
      }

      if sigint {
        let _ = signal::kill(id, 2); // SIGINT
      } else if sigterm {
        let _ = signal::kill(id, 15); //SIGTERM
      }
    });

    // TODO bind those handlers only once
    let sigint_action = signal::SigAction::new(
      handle_sigint, signal::SockFlag::empty(), signal::SigSet::empty()
    );
    let sigterm_action = signal::SigAction::new(
      handle_sigterm, signal::SockFlag::empty(), signal::SigSet::empty()
    );
    unsafe {
      let _ = signal::sigaction(signal::SIGINT, &sigint_action);
      let _ = signal::sigaction(signal::SIGTERM, &sigterm_action);
    }

    let status = child.wait().unwrap();
    if status.success() {
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Other, "Exited with non-zero code"))
    }
  }
}
