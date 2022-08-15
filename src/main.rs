#![allow(dead_code)]
#![allow(unused_imports)]

mod output_status;
mod lid_status;
mod swaysock;
extern crate edid_rs;

use swaysock::ensure_swaysock;
use std::{cmp::{self, Ordering}, env, process::exit};
use lid_status::{inspect_lid, LidState};
use output_status::{inspect_outputs, Output, OutputStatus};
use swayipc::{Connection, Fallible};

fn main() -> Fallible<()> {
  ensure_swaysock();
  let output_status = inspect_outputs();

  match inspect_lid().state {
    LidState::Open => open_lid(&output_status.e_dp1)?,
    LidState::Closed => closed_lid(&output_status.e_dp1)?,
    _ => print!("I don't know if it's open or not :("),
  }

  // let output_count = output_status.count();
  // match &output_count.cmp(&1) {
  //   Ordering::Greater => println!("There's {} enabled displays connected!", output_count),
  //   Ordering::Equal => println!("just the one screen."),
  //   Ordering::Less => println!("no screens?"),
  // }

  Ok(())
}

fn open_lid(edp1: &Output) -> Fallible<()> {
  match edp1.enabled {
    true => print!("we good, "),
    false => {
      print!("wakey pakey, ");
      ipc_command("output eDP-1 enable")?
    }
  }

  Ok(())
}

fn closed_lid(edp1: &Output) -> Fallible<()>{
  match edp1.enabled {
    false => print!("sleepin already"),
    true => {
      print!("Time to sleep, ");
      ipc_command("output eDP-1 disable")?
    }
  }

  Ok(())
}

fn ipc_command(command: &str) -> Fallible<()> {
    let mut connection = Connection::new()?;

    for outcome in connection.run_command(command)? {
        if let Err(error) = outcome {
            println!("failure '{}'", error);
            return Err(error);
        }
    }

    Ok(())
}
