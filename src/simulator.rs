#[derive(Debug)]
pub struct Simulator {
    name: String,
    version: String,
}

impl Simulator {
    pub fn new(name: String, version: String) -> Simulator {
        Simulator {
            name,
            version,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn tag(&self) -> String {
        String::from(format!("{}:{}", self.name, self.version))
    }
}
