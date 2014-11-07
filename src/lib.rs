use std::collections::HashSet;
use std::collections::HashMap;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::io::{File, IoResult, USER_RWX};

pub type Matches = HashSet<String>;
pub type AttributeMatches = HashMap<String, Matches>;

#[allow(dead_code)]
pub struct InputPort(&'static str);
pub static INPUT_AUTO: InputPort = InputPort("");
pub static INPUT_1: InputPort = InputPort("in1");
pub static INPUT_2: InputPort = InputPort("in2");
pub static INPUT_3: InputPort = InputPort("in3");
pub static INPUT_4: InputPort = InputPort("in4");

pub struct OutputPort(&'static str);
pub static OUTPUT_AUTO: OutputPort = OutputPort("");
pub static OUTPUT_A: OutputPort = OutputPort("outA");
pub static OUTPUT_B: OutputPort = OutputPort("outB");
pub static OUTPUT_C: OutputPort = OutputPort("outC");
pub static OUTPUT_D: OutputPort = OutputPort("outD");

struct Device {
    path: Option<Path>,
    device_index: int,
}

impl Device {
    fn new() -> Device {
        Device { path: None, device_index: -1 }
    }

    fn get_attr_string(&self, name: &str) -> Option<String> {
        match self.path {
            None => None,
            Some(ref path) => File::open(&path.join(name)).and_then(
                |mut f| { f.read_to_string().map(
                    |mut text| { text.trim().to_string()}) }).ok(),
        }
    }

    fn get_attr_int(&self, name: &str) -> Option<int> {
        match self.get_attr_string(name) {
            None => None,
            Some(text) => from_str(text.as_slice()),
        }
    }

    fn connect(&mut self, dir: &Path, pattern: &str,
               match_spec: AttributeMatches) -> bool {
        let mut paths = match fs::walk_dir(dir) {
            Err(_) => { return false; }
            Ok(paths) => paths,
        };
        let mut is_match = true;
        for path in paths.filter(|e| {
            e.dir_path() == *dir &&
            e.filename_str().unwrap().starts_with(pattern)
        }) {
            self.path = Some(path.clone());
            println!("trying path {}", path.display());
            for (k, v) in match_spec.iter() {
                let value = self.get_attr_string(k.as_slice()).unwrap();
                println!("k,matches,value {},{},{}", k, v, value);
                println!("contains? {}", v.contains(&value));
                if !v.contains(&value) {
                    is_match = false;
                    self.path = None;
                    break;
                }
            }
        }
        is_match
    }
}

pub struct Sensor {
    dev: Device,
}

impl Sensor {
    pub fn new(port: InputPort) -> Sensor {
        // stub.
        Sensor { dev: Device::new() }
    }
}

#[cfg(test)]
mod test {
    extern crate hamcrest;
    use super::Device;
    use std::collections::{HashSet, HashMap};
    
    #[test]
    fn try_types() {
        let mut matches = HashSet::new();
        matches.insert("Linux");
    }

    #[test]
    fn device_connect() {
        let mut dut = Device::new();
        let mut matchy = HashMap::new();
        let mut matches = HashSet::new();
        matches.insert("in1".to_string());
        matchy.insert("port_name".to_string(), matches);
        let data_dir = Path::new(file!()).dir_path().dir_path().join("data");
        let sensor_dir = data_dir.join_many(&["sys", "class", "msensor"]);
        assert!(dut.connect(&sensor_dir, "sensor", matchy));
    }
}
