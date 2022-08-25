mod lid_status;
mod sway;

use sway::Sway;
use lid_status::{inspect_lid, LidState};
use swayipc::Error;

fn main() -> Result<(), Error> {
  let mut state = SwayLid::new()?;

  state.toggle_lid()?;

  Ok(())
}

struct SwayLid {
  sway: Sway,
  lid_status: LidState
}

impl SwayLid {
  pub fn new() -> Result<Self, Error> {
    Ok(Self {
      sway: Sway::new()?,
      lid_status: inspect_lid().state
    })
  }

  pub fn toggle_lid(&mut self) -> Result<(), Error> {
    let outputs = self.sway.outputs()?;
    match self.lid_status {
        LidState::Open => self.sway.enable(outputs.internal()?),
        LidState::Closed => self.sway.disable(outputs.internal()?),
        LidState::Unknown => Ok(()),
    }
  }
}

