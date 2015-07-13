use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::io::{Error, Write};

use bases::{get_base, GitBase};
use cli::Args;

pub fn run(args: Args) {
  let app_name = args.arg_name;
  let base = get_base(&args.arg_base);
  let workdir = get_workdir(&app_name, &args.flag_directory).unwrap();

  println!("Scaffolding new app:");
  println!("  name: {:}", app_name);
  println!("  base: {:}", base.git_url());
  println!("  workdir: {:}", &workdir);

  println!("Creating the working directory...");
  create_workdir(&workdir).unwrap();
  println!("Importing base image...");
  import_base(&workdir, &base).unwrap();
  remove_base_git_dir(&workdir).unwrap();

  if !validate_app_name(&workdir, &app_name).unwrap() {
    panic!("Invalid app name!");
  }

  println!("Making data directories...");
  make_data_dirs(&workdir).unwrap();
  println!("Bootstrapping the app...");
  bootstrap_workdir(&workdir, &app_name).unwrap();
  println!("Writing metadata...");
  write_app_name_file(&workdir, &app_name).unwrap();
  write_app_base_file(&workdir, &base).unwrap();
  println!("Done!");
}

fn get_workdir(app_name: &str, directory: &Option<String>) -> Result<String, Error> {
  env::current_dir().map(|mut path| {
    match directory {
      &Some(ref d) => path.push(d),
      &None => path.push(app_name)
    }
    path
  }).map(|path| {
    path.into_os_string().into_string().unwrap()
  })
}

fn create_workdir(workdir: &str) -> Result<(), &str> {
  // TODO use std::path::Path::exists when it's declared as stable
  match fs::metadata(workdir) {
    Ok(_) => {
      Err("Workdir already exists")
    },
    Err(_) => {
      fs::create_dir(workdir)
        .map(|_| { () })
        .map_err(|_| { "Could not create the workdir" })
    }
  }
}

fn import_base(workdir: &str, base: &GitBase) -> Result<ExitStatus, Error> {
  let mut apiece_path_buf = PathBuf::from(workdir);
  apiece_path_buf.push("apiece.io");
  let apiece_path = apiece_path_buf.into_os_string();

  let mut command = Command::new("git");
  command.arg("clone");
  command.arg("--depth=1");
  match &base.branch {
    &Some(ref branch) => {
      command.arg("--branch");
      command.arg(&branch);
    },
    &None => {}
  }
  command.arg(&base.repo);
  command.arg(apiece_path);

  command.status()
}

fn remove_base_git_dir(workdir: &str) -> Result<(), Error> {
  let mut base_git_dir = PathBuf::from(workdir);
  base_git_dir.push("apiece.io");
  base_git_dir.push(".git");

  fs::remove_dir_all(base_git_dir)
}

fn validate_app_name(workdir: &str, app_name: &str) -> Result<bool, Error> {
  let mut validate_script_path = PathBuf::from(&workdir);
  validate_script_path.push("apiece.io");
  validate_script_path.push("bootstrap");
  validate_script_path.push("validate");

  // TODO use std::path::Path::exists when it's declared as stable
  match fs::metadata(&validate_script_path) {
    Ok(_) => {
      Command::new(&validate_script_path)
        .arg(app_name)
        .status()
        .map(|status| { status.success() })
    }
    Err(_) => {
      Ok(true)
    }
  }
}

fn make_data_dirs(workdir: &str) -> Result<(), Error> {
  make_data_dir(workdir, "production").and_then(|_| {
    make_data_dir(workdir, "development")
  }).and_then(|_| {
    make_data_dir(workdir, "local")
  }).and_then(|_| {
    make_data_dir(workdir, "test")
  })
}

fn make_data_dir(workdir: &str, env: &str) -> Result<(), Error> {
  let mut new_dir_path = PathBuf::from(workdir);
  new_dir_path.push("apiece.io");
  new_dir_path.push("data");
  new_dir_path.push(env);
  fs::create_dir_all(new_dir_path)
}

fn bootstrap_workdir(workdir: &str, app_name: &str) -> Result<(), Error> {
  let mut bootstrap_path = PathBuf::from(workdir);
  bootstrap_path.push("apiece.io/bootstrap");
  let mut bootstrap_script_path = bootstrap_path.to_path_buf();
  bootstrap_script_path.push("run");

  let mut command = Command::new(bootstrap_script_path);
  command.current_dir(workdir);
  command.env("APIECEIO_PACKAGE_NAME", app_name);
  command.status().and_then(|_| {
    fs::remove_dir_all(&bootstrap_path)
  })
}

fn write_app_name_file(workdir: &str, app_name: &str) -> Result<(), Error> {
  write_file(workdir, "apiece.io/app-name", app_name)
}

fn write_app_base_file(workdir: &str, base: &GitBase) -> Result<(), Error> {
  write_file(workdir, "apiece.io/app-base", &base.git_url())
}

fn write_file(workdir: &str, path: &str, data: &str) -> Result<(), Error> {
  let mut file_path = PathBuf::from(workdir);
  file_path.push(path);
  fs::File::create(file_path).and_then(|mut f| {
    f.write_all(data.as_bytes())
  })
}
