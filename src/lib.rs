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
    path: String,
    device_index: int,
}

impl Device {
    fn new() -> Device {
        Device { path: String::new(), device_index: -1 }
    }

    fn get_attr_string(&self, name: &str) -> String {
        let mut f = File::open(&Path::new(self.path + name));
        f.read_to_string().ok().expect("no device connected")
    }

    fn connect(&mut self, dir: &str, pattern: &str,
               match_spec: AttributeMatches) -> bool {
        let mut paths = match fs::walk_dir(&Path::new(dir)) {
            Err(_) => { return false; }
            Ok(paths) => paths,
        };
        for path in paths.filter(|e| { true }) {
            println!("{}", path.display());
        }
        // stub
        false
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
        assert!(dut.connect(sensor_dir.as_str().expect(""), "sensor", matchy));
    }
}
