extern crate serde_yaml;

use crate::domain::source::model::Source;
use eyre::Result;
use std::collections::HashMap;
use std::fs::{File, copy};
use std::path::{PathBuf, Path};

pub struct SourceRepository {
    path: String,
    bck_path: String,
    data: HashMap<String, Source>,
}

impl SourceRepository {
    pub fn new(path: String) -> Result<SourceRepository> {
        let mut bck_path = path.clone();
        bck_path.push_str(".bck");
        Ok(SourceRepository {
            path: path.clone(),
            bck_path,
            data: if PathBuf::from(&path).exists() {
                serde_yaml::from_reader(File::open(&path)?)?
            } else {
                HashMap::new()
            },
        })
    }

    fn dump(&self) -> Result<()> {
        if Path::new(&self.path).exists() {
            copy(&self.path, &self.bck_path)?;
        }
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
