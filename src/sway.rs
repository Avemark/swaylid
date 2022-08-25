mod swaysock;
mod outputs;

use swayipc::{Connection, Error, Output};
use outputs::Outputs;

pub struct Sway {
  connection: Connection
}

impl Sway {
  pub fn new() -> Result<Self, Error> {
    swaysock::ensure_swaysock();

    let connection = Connection::new()?;

    Ok( Self { connection } )
  }

  pub fn outputs(&mut self) -> Result<Outputs, Error> {
    Ok(
      Outputs::new(self.connection.get_outputs()?)
    )
  }

  pub fn enable(&self, output: &Output) -> Result<(), Error> {
    if !output.active {
      println!("enabling!");
      Self::ipc_command(&format!("output {} enable", output.name))?
    }
    Ok(())
  }

  pub fn disable(&self, output: &Output) -> Result<(), Error> {
    if output.active {
      println!("Disabling!<");
      Self::ipc_command(&format!("output {} disable", output.name))?
    }
    Ok(())
  }


  fn ipc_command(command: &str) -> Result<(), Error> {
    let mut connection = Connection::new()?;

    for outcome in connection.run_command(command)? {
        if let Err(error) = outcome {
          println!("failure '{}'", error)
        }
    }

    Ok(())
  }
}