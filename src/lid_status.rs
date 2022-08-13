

use std::{fs::read_to_string};


#[derive(Debug)]
pub struct LidStatus {
  pub state: LidState
}

impl LidStatus {
  pub fn open(self) -> bool {
    self.state == LidState::Open
  }

  pub fn _closed(self) -> bool {
    self.state == LidState::Closed
  }
}

impl From<String> for LidStatus {
  fn from(item: String) -> Self {
    LidStatus {
      state: LidState::from(item.as_str()),
    }
  }
}

impl From<&str> for LidStatus {
  fn from(item: &str) -> Self {
    LidStatus {
      state: LidState::from(item),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LidState {
  Open,
  Closed,
  Unknown
}

impl From<&str> for LidState {
  fn from(item: &str) -> Self {
    match item {
      "open" => LidState::Open,
      "closed" => LidState::Closed,
      _ => LidState::Unknown
    }
  }
}

pub fn inspect_lid() -> LidStatus {
  match read_to_string("/proc/acpi/button/lid/LID/state") {
    Ok(str) => {
      match str.split_whitespace().next_back() {
        Some(str) => LidStatus::from(LidStatus::from(str)),
        None => LidStatus::from("")
      }
    },
    Err(_) => LidStatus::from(""),
  }
}