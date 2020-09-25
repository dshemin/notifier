extern crate serde_yaml;

use crate::domain::source::model::Source;
use eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

pub struct SourceRepository {
    path: String,
    data: HashMap<String, Source>,
}

impl SourceRepository {
    pub fn new(path: String) -> Result<SourceRepository> {
        Ok(SourceRepository {
            path: path.clone(),
            data: if PathBuf::from(&path).exists() {
                serde_yaml::from_reader(File::open(&path)?)?
            } else {
                HashMap::new()
            },
        })
    }

    fn dump(&self) -> Result<()> {
        serde_yaml::to_writer(File::create(&self.path)?, &self.data)?;
        Ok(())
    }

    pub fn save(&mut self, s: &Source) -> Result<()> {
        self.data.insert(s.name().clone(), s.clone());
        self.dump()
    }

    pub fn list(&self) -> Result<Vec<&Source>> {
        Ok(self.data.iter().map(|(_, s)| s).collect())
    }
}
