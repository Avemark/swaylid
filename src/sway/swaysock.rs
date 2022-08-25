use std::{process::{Command, Stdio}, io::Read};

use swayipc::Error;

static SOCK_ENV_KEY: &str = "SWAYSOCK";

pub fn ensure_swaysock() {
  match std::env::var(SOCK_ENV_KEY) {
    Ok(_) => return,
    Err(_) => (),
  }

  match socket_path() {
    Ok(path) => std::env::set_var(SOCK_ENV_KEY, path),
    Err(_) => (),
  }
}

fn socket_path() -> Result<String, Error> {
  Ok(
    format!(
      "/run/user/{uid}/sway-ipc.{uid}.{pid}.sock",
      uid = ext_command("id", Some("-u"))?,
      pid = ext_command("pidof", Some("sway"))?,
    )
  )
}

fn ext_command<'a>(cmd: &'a str, arg: Option<&str>) -> Result<String, Error> {
  let mut command = Command::new(cmd);
  let mut child = apply_arg(&mut command, arg).stdout(Stdio::piped()).spawn()?;

  let mut buf = String::new();
  if let Some(mut stdout) = child.stdout.take() {
      stdout.read_to_string(&mut buf)?;
  }
  child.wait()?;

  Ok(String::from(buf.trim()))
}

fn apply_arg<'a>(command: &'a mut Command, arg: Option<&str>) -> &'a mut Command {
  match arg {
      Some(str) => command.arg(str),
      None => command
  }
}
