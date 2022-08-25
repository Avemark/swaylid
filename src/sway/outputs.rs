use swayipc::{Output, Error};

pub struct Outputs {
  outputs: Vec<Output>
}


impl Outputs {
  pub fn new(outputs: Vec<Output>) -> Self {
    Self { outputs }
  }
  pub fn _external(&self) -> Vec<&Output> {
    (&self.outputs).into_iter().filter(
      |output| output.name != "eDP-1"
    ).collect()
  }

  pub fn internal(&self) -> Result<&Output, Error> {
    Ok(
      (&self.outputs).into_iter().find(
        |output| { output.name == "eDP-1" }
      ).as_ref().unwrap()
    )
  }
}