use std::{fs::{read, read_to_string}, io::Cursor};
use edid_rs::EDID;

#[derive(Clone, Debug)]
pub struct OutputStatus {
    pub e_dp1: Output,
    pub dp1: Output,
    pub dp2: Output,
    pub dp3: Output,
    pub dp4: Output,
    pub dp5: Output,
    pub dp6: Output,
    pub dp7: Output,
    pub dp8: Output,
    pub dp9: Output,
}

#[derive(Debug, Clone)]
pub struct Output {
  pub edid: Option<EDID>,
  pub enabled: bool,
}


impl OutputStatus {
    pub fn count(&self) -> usize {
      self.all().len()
    }

    fn all(&self) -> Vec<Output> {
        let mut collection = Vec::<Output>::new();

        for output in [
            &self.e_dp1, &self.dp1, &self.dp2, &self.dp3, &self.dp4, &self.dp5, &self.dp6, &self.dp7,
            &self.dp8, &self.dp9,
        ] {
            match output.enabled {
                true => collection.push(output.clone()),
                false => (),
            }
        }
        return collection
    }
}

impl IntoIterator for OutputStatus {
  type Item = Output;
  type IntoIter = std::vec::IntoIter<Output>;

  fn into_iter(self) -> Self::IntoIter {
    return self.all().into_iter()
  }
}

impl Into<Vec<Output>> for OutputStatus {
  fn into(self) -> Vec<Output> {
    self.all()
  }
}

pub fn inspect_outputs() -> OutputStatus {
  return OutputStatus {
      e_dp1: get_output(String::from("eDP-1")),
      dp1: get_output(String::from("DP-1")),
      dp2: get_output(String::from("DP-2")),
      dp3: get_output(String::from("DP-3")),
      dp4: get_output(String::from("DP-4")),
      dp5: get_output(String::from("DP-5")),
      dp6: get_output(String::from("DP-6")),
      dp7: get_output(String::from("DP-7")),
      dp8: get_output(String::from("DP-8")),
      dp9: get_output(String::from("DP-9")),
  };
}

fn get_output(output_key: String) -> Output {
  let base_path = format!("/sys/class/drm/card0-{}/", output_key);


  let enabled = match read_to_string(format!("{}{}", &base_path, "enabled")) {
    Ok(status) => {
      match status.as_str().trim() {
        "enabled" => true,
        _ => return Output { edid: None, enabled: false },
      }
    },
    Err(_) => return Output { edid: None, enabled: false },
  };

  let edid = match readfile(&base_path, "edid") {
    Some(bytes) => match edid_rs::parse(&mut Cursor::new(bytes)) {
        Ok(edid) => Some(edid),
        Err(error) => {
          println!("Failure!: {}\nwhile reading data for {}", error, output_key);
          None
        },
      },
    None => None
  };

  Output { edid, enabled }
}

fn readfile(base_path: &str, file: &str) -> Option<Vec<u8>> {
  let path = format!("{}{}", base_path, file);
  match read(&path) {
    Ok(bytes) => Some(bytes),
    Err(error) => {
        println!("Failure!: {}\nCould not read file {}", error, path);
        None
      },
  }
}