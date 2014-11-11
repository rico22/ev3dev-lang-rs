use std::collections::HashSet;
use std::collections::HashMap;
use std::io::fs;
use std::io::{File, Append, Write};
use std::num;

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

#[allow(dead_code)]
struct Device {
    path: Option<Path>,
    device_index: int,
}

#[allow(dead_code)]
impl Device {
    fn new() -> Device {
        Device { path: None, device_index: -1 }
    }

    fn get_attr_string(&self, name: &str) -> Option<String> {
        match self.path {
            None => None,
            Some(ref path) => File::open(&path.join(name)).and_then(
                |mut f| { f.read_to_string().map(
                    |text| { text.trim().to_string()}) }).ok(),
        }
    }

    fn set_attr_string(&self, name: &str, value: &str) -> Option<()> {
        match self.path {
            None => None,
            Some(ref path) => File::open_mode(
                &path.join(name), Append, Write).and_then(
                |mut f| { f.write_str(value)}).ok(),
        }
    }

    fn get_attr_int(&self, name: &str) -> Option<int> {
        match self.get_attr_string(name) {
            None => None,
            Some(text) => from_str(text.as_slice()),
        }
    }

    fn set_attr_int(&self, name: &str, value: int) -> Option<()> {
        self.set_attr_string(name, format!("{}", value).as_slice())
    }

    fn _parse_device_index(&self) -> Option<int> {
        match self.path {
            None => None,
            Some(ref path) => from_str(path.filename_str().map(
                |e| { e.trim_left_chars(
                    |c: char| { !c.is_digit() }) }).unwrap()),
        }
    }

    fn device_index(&mut self) -> Option<int> {
        if self.device_index < 0 {
            return match self._parse_device_index() {
                None => None,
                Some(index) => {
                    self.device_index = index;
                    return Some(index);
                }
            }
        }
        Some(self.device_index)
    }

    fn connect(&mut self, dir: &Path, pattern: &str,
               match_spec: AttributeMatches) -> Option<()> {
        let paths = match fs::walk_dir(dir) {
            Err(_) => {
                println!("dir walk error");
                return None;
            }
            Ok(paths) => paths,
        };
        let mut is_match = Some(());
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
                    is_match = None;
                    self.path = None;
                    break;
                }
            }
        }
        is_match
    }
}

pub trait SystemShim {
    fn root_path(&self) -> Path { Path::new("/") }
}

#[allow(dead_code)]
struct Ev3DevSystem;

#[allow(dead_code)]
impl SystemShim for Ev3DevSystem {
    fn root_path(&self) -> Path {
        return Path::new("/");
    }
}
    
#[allow(dead_code)]
static SENSOR_CLASS_DIR: &'static str = "sys/class/msensor";
#[allow(dead_code)]
static SENSOR_PATTERN: &'static str = "sensor";

#[allow(dead_code)]
pub struct Sensor {
    dev: Device,
    port_name: Option<String>,
    type_name: Option<String>,
    mode: Option<String>,
    //modes: Option<HashSet>,
    nvalues: Option<int>,
    dp: Option<int>,
    dp_scale: f64,
}

#[allow(dead_code)]
impl Sensor {
    // non-public internal machinery.

    fn new() -> Sensor {
        // stub.
        Sensor { dev: Device::new(), port_name: None, type_name: None,
        mode: None, nvalues: None, dp: None, dp_scale: 0f64}
    }

    fn connect<S: SystemShim>(&mut self, system: &S,
               match_spec: AttributeMatches) -> Option<()> {
        match self.dev.connect(&system.root_path().join(SENSOR_CLASS_DIR),
                               SENSOR_PATTERN, match_spec) {
            None => None,
            Some(_) => {
                println!("sensor connect ok");
                self.init_binding();
                self.init_members();
                return Some(());
            }
        }
    }

    fn init_binding(&mut self) {
        self.port_name = self.dev.get_attr_string("port_name");
        self.type_name = self.dev.get_attr_string("name");
        println!("sensor init binding ok");
    }

    fn init_members(&mut self) {
        self.mode = self.dev.get_attr_string("mode");
        //self.modes = self.dev.get_attr_set("modes");
        self.nvalues = self.dev.get_attr_int("num_values");
        self.dp = self.dev.get_attr_int("dp");

        let dpi = self.dp.unwrap();
        println!("sensor dpi ok");
        
        let dpu = dpi.to_uint().unwrap();
        println!("sensor dpu ok");
        self.dp_scale = num::pow(1e-1f64, dpu);
        println!("sensor init members ok");
    }

    pub fn from_port<S: SystemShim>(
        system: &S, port: &InputPort) -> Option<Sensor> {
        let mut sensor = Sensor::new();

        let mut match_spec = HashMap::new();
        let mut matches = HashSet::new();
        let &InputPort(port_string) = port;
        matches.insert(port_string.to_string());
        match_spec.insert("port_name".to_string(), matches);

        match sensor.connect(system, match_spec) {
            None => None,
            Some(_) => Some(sensor),
        }
    }

    pub fn from_port_and_type<S: SystemShim>(system: &S,
        port: &InputPort, sensor_types: &HashSet<String>) -> Option<Sensor> {
        let mut sensor = Sensor::new();

        let mut match_spec = HashMap::new();
        let mut ports = HashSet::new();
        let &InputPort(port_string) = port;
        ports.insert(port_string.to_string());
        match_spec.insert("port_name".to_string(), ports);
        match_spec.insert("name".to_string(), sensor_types.clone());

        match sensor.connect(system, match_spec) {
            None => None,
            Some(_) => Some(sensor),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate hamcrest;
    use super::Device;
    use super::SystemShim;
    use std::collections::{HashSet, HashMap};

    struct TestSystem;

    impl SystemShim for TestSystem {
        fn root_path(&self) -> Path {
            return Path::new(file!()).dir_path().dir_path().join("data");
        }
    }
    
    #[test]
    fn try_types() {
        let mut matches = HashSet::new();
        matches.insert("Linux");
    }

    #[test]
    fn device_basics() {
        let system = TestSystem;
        let mut dut = Device::new();
        let mut matchy = HashMap::new();
        let mut matches = HashSet::new();
        matches.insert("in1".to_string());
        matchy.insert("port_name".to_string(), matches);
        let sensor_dir = system.root_path().join_many(
            &["sys", "class", "msensor"]);
        assert!(dut.connect(&sensor_dir, "sensor", matchy) == Some(()));
        assert!(dut.device_index() == Some(0));
        assert!(dut.get_attr_int("value0") == Some(0));
    }

    #[test]
    fn sensor_basics() {
        let system = TestSystem;
        assert!(super::Sensor::from_port(&system, &super::INPUT_1).is_some());
    }
}
