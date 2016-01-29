// #![feature(plugin)]

// #![plugin(clippy)]


use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::ops::Deref;
use std::io::Result;

pub type Matches = HashSet<String>;
pub type AttributeMatches = HashMap<String, Matches>;

//#[allow(dead_code)]
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

//#[allow(dead_code)]
struct Device {
    path: PathBuf,
    device_index: Option<isize>,
}

//#[allow(dead_code)]
impl Device {
    fn new() -> Device {
        Device {
            path: PathBuf::new(),
            device_index: None,
        }
    }

    fn get_attr_string(&self, name: &str) -> Result<String> {
        // assert!(self.path.deref().is_dir());
        let mut s = String::new();
        try!(File::open(&self.path.join(name))
                 .and_then(|mut f| f.read_to_string(&mut s)));
        Ok(s.trim().to_owned())
    }

    fn set_attr_string(&self, name: &str, value: &str) -> Result<()> {
        // assert!(self.path.is_dir());
        OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.path.join(name))
            .and_then(|mut f| write!(&mut f, "{}", value))
    }

    fn get_attr_int(&self, name: &str) -> Result<isize> {
        let text = try!(self.get_attr_string(name));
        Ok(text.parse::<isize>().unwrap())
    }

    // This dead-code flag seems unnecessary. Bug?
    #[allow(dead_code)]
    fn set_attr_int(&self, name: &str, value: isize) -> Result<()> {
        self.set_attr_string(name, &format!("{}", value))
    }

    fn get_attr_set(&self, name: &str) -> Result<HashSet<String>> {
        let text = try!(self.get_attr_string(name));
        let mut set = HashSet::<String>::new();
        for x in text.trim().split(' ') {
            set.insert(x.to_owned());
        }
        Ok(set)
    }

    fn _parse_device_index(&self) -> isize {
        self.path
            .deref()
            .file_name()
            .map(|e| {
                e.to_str()
                 .expect("ZOMG!")
                 .trim_left_matches(|c: char| !c.is_digit(10u32))
            })
            .unwrap()
            .parse::<isize>()
            .unwrap()
    }

    // This dead-code flag seems unnecessary. Bug?
    #[allow(dead_code)]
    fn get_device_index(&mut self) -> isize {
        if self.device_index.is_none() {
            self.device_index = Some(self._parse_device_index());
        }
        self.device_index.unwrap()
    }

    fn connect(&mut self,
               dir: &Path,
               pattern: &str,
               match_spec: AttributeMatches)
               -> Option<()> {
        let paths = match fs::read_dir(dir) {
            Err(_) => {
                println!("dir walk error");
                return None;
            }
            Ok(paths) => paths,
        };
        let mut is_match = Some(());
        for path in paths.filter(|e| e.is_ok()) {
            self.path = path.unwrap().path().clone();
            if !self.path
                    .to_str()
                    .expect("ZOUNDS!")
                    .starts_with(pattern) {
                continue;
            }
            println!("trying path {}", self.path.display());
            for (k, v) in &match_spec {
                let value = self.get_attr_string(k).unwrap();
                println!("k,matches,value {},{}", k, value);
                println!("contains? {}", v.contains(&value));
                if !v.contains(&value) {
                    is_match = None;
                    self.path = PathBuf::new();
                    break;
                }
            }
        }
        is_match
    }
}

pub trait SystemShim {
    fn root_path(&self) -> PathBuf;
}

#[allow(dead_code)]
struct Ev3DevSystem;

//#[allow(dead_code)]
impl SystemShim for Ev3DevSystem {
    fn root_path(&self) -> PathBuf {
        PathBuf::from("/")
    }
}

//#[allow(dead_code)]
static SENSOR_CLASS_DIR: &'static str = "sys/class/msensor";
//#[allow(dead_code)]
static SENSOR_PATTERN: &'static str = "sensor";

//#[allow(dead_code)]
pub struct Sensor {
    dev: Device,
    port_name: String,
    type_name: String,
    mode: String,
    modes: HashSet<String>,
    nvalues: isize,
    dp: isize,
    dp_scale: f64,
}

//#[allow(dead_code)]
impl Sensor {
    // non-public internal machinery.

    fn new() -> Sensor {
        // stub.
        Sensor {
            dev: Device::new(),
            port_name: String::new(),
            type_name: String::new(),
            mode: String::new(),
            modes: HashSet::new(),
            nvalues: 0,
            dp: -1,
            dp_scale: 0f64,
        }
    }

    fn connect<S: SystemShim>(&mut self,
                              system: &S,
                              match_spec: AttributeMatches)
                              -> Option<()> {
        match self.dev.connect(&system.root_path().join(SENSOR_CLASS_DIR),
                               SENSOR_PATTERN,
                               match_spec) {
            None => None,
            Some(_) => {
                self.init_binding();
                self.init_members();
                Some(())
            }
        }
    }

    fn init_binding(&mut self) {
        self.port_name = self.dev.get_attr_string("port_name").unwrap();
        self.type_name = self.dev.get_attr_string("name").unwrap();
        println!("sensor init binding ok");
    }

    fn init_members(&mut self) {
        self.mode = self.dev.get_attr_string("mode").unwrap();
        self.modes = self.dev.get_attr_set("modes").unwrap();
        self.nvalues = self.dev.get_attr_int("num_values").unwrap();
        self.dp = self.dev.get_attr_int("dp").unwrap();

        let dpi = self.dp;
        println!("sensor dpi ok");

        let dpu = dpi as i32;
        println!("sensor dpu ok");
        self.dp_scale = (1e-1f64).powi(dpu);
        println!("sensor init members ok");
    }

    pub fn from_port<S: SystemShim>(system: &S,
                                    port: &InputPort)
                                    -> Option<Sensor> {
        let mut sensor = Sensor::new();

        let mut match_spec = HashMap::new();
        let mut matches = HashSet::new();
        let &InputPort(port_string) = port;
        matches.insert(port_string.to_owned());
        match_spec.insert("port_name".to_owned(), matches);

        match sensor.connect(system, match_spec) {
            None => None,
            Some(_) => Some(sensor),
        }
    }

    pub fn from_port_and_type<S: SystemShim>(system: &S,
                                             port: &InputPort,
                                             sensor_types: &HashSet<String>)
                                             -> Option<Sensor> {
        let mut sensor = Sensor::new();

        let mut match_spec = HashMap::new();
        let mut ports = HashSet::new();
        let &InputPort(port_string) = port;
        ports.insert(port_string.to_owned());
        match_spec.insert("port_name".to_owned(), ports);
        match_spec.insert("name".to_owned(), sensor_types.clone());

        match sensor.connect(system, match_spec) {
            None => None,
            Some(_) => Some(sensor),
        }
    }

    pub fn units(&self) -> String {
        self.dev.get_attr_string("units").unwrap()
    }

    pub fn set_mode(&mut self, mode: &str) {
        if self.mode != mode {
            self.dev.set_attr_string("mode", mode).unwrap();
            self.init_members();
        }
    }

    pub fn value(&self, index: isize) -> isize {
        assert!(index < self.nvalues && index >= 0);
        self.dev.get_attr_int(&format!("value{}", index)).unwrap()
    }

    pub fn float_value(&self, index: isize) -> f64 {
        self.value(index) as f64 * self.dp_scale
    }
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use super::Device;
    use super::SystemShim;
    use std::collections::{HashSet, HashMap};
    use std::path::PathBuf;
    use std::fs::{DirBuilder, File};
    use std::io::prelude::*;

    pub struct TestSystem {
        dir: tempdir::TempDir,
    }

    impl SystemShim for TestSystem {
        fn root_path(&self) -> PathBuf { self.dir.path().to_path_buf() }
    }

    pub trait TestCase {
        fn setup(&mut self);
    }

    fn init_file(path: &PathBuf, name: &str, value: &[u8]) {
        let fname = path.join(name);
        println!("fname {}", fname.display());
        File::create(&fname).and_then(|mut f| f.write_all(value))
            .expect("bad write");
    }


    impl TestCase for TestSystem {
        fn setup(&mut self) {
            let path = self.root_path()
                .join("sys").join("class").join("msensor").join("sensor0");
            println!("path {}", path.display());
            DirBuilder::new().recursive(true)
                .create(&path).expect("bad dir");

            init_file(&path, "modes", b"TOUCH");
            init_file(&path, "mode", b"TOUCH");
            init_file(&path, "port_name", b"in1");
            init_file(&path, "name", b"lego-ev3-touch");
            init_file(&path, "num_values", b"1");
            init_file(&path, "value0", b"0");
            init_file(&path, "dp", b"0");
        }
    }

    macro_rules! test {
        // TODO rico: add a test fixture struct?
        ($name:ident $fixture:ident $expr:expr) => (
            #[test]
            fn $name() {
                let mut $fixture = TestSystem {
                    dir: tempdir::TempDir::new("").expect("bad tempdir")
                };
                $fixture.setup();
                $expr;
            }
        )
    }

    #[test]
    fn try_types() {
        let mut matches = HashSet::new();
        matches.insert("Linux");
    }

    test!(device_basics system {
        let mut dut = Device::new();
        let mut matchy = HashMap::new();
        let mut matches = HashSet::new();
        matches.insert("in1".to_owned());
        matchy.insert("port_name".to_owned(), matches);
        let sensor_dir = system.root_path()
                               .join("sys")
                               .join("class")
                               .join("msensor");
        assert!(dut.connect(&sensor_dir, "sensor", matchy).is_some());
        assert!(dut.get_device_index() == 0);
        assert!(dut.set_attr_int("value0", 1).is_ok());
        assert!(dut.get_attr_int("value0").unwrap() == 1);
    });

    test!(sensor_basics system {
        let sens1 = super::Sensor::from_port(&system, &super::INPUT_1);
        println!("got here");
        assert!(sens1.is_some());
        let super::InputPort(port1) = super::INPUT_1;
        assert!(sens1.unwrap().port_name == port1);
    });
}
