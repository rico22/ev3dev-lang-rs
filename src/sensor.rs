use device::{Device, AttributeMatches, InputPort};
use system::SystemShim;
use std::collections::HashMap;
use std::collections::HashSet;

static SENSOR_CLASS_DIR: &'static str = "sys/class/msensor";
static SENSOR_PATTERN: &'static str = "sensor";

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

    use device;
    use system::SystemShim;
    use testbase::{TestCase, TestSystem, init_file};
    use std::fs::DirBuilder;

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

    test!(sensor_basics system {
        let sens1 = super::Sensor::from_port(&system, &device::INPUT_1);
        println!("got here");
        assert!(sens1.is_some());
        let device::InputPort(port1) = device::INPUT_1;
        assert!(sens1.unwrap().port_name == port1);
    });
}
