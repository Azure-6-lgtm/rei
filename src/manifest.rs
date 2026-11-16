use crate::error;
use std::collections::HashSet;

pub struct Manifest {
    name: String,
    source: String,
    version: String,
    checksum: String,
    deps: Option<String>,
    author: Option<String>,
    description: Option<String>,
}

impl Manifest {  
    fn validate_manifest(&self) -> bool {
        if self.name.is_empty() {
            false
        } else if self.source.is_empty() {
            false
        } else if self.version.is_empty() {
            false
        } else if self.checksum.is_empty() {
            false
        } else {
            true
        }
    }

    pub fn parse_manifest(manifest: &str) -> Result<Self, error::Error> {
        let mut result = Self {
            name: String::new(),
            source: String::new(),
            version: String::new(),
            checksum: String::new(),
            deps: None,
            author: None,
            description: None, 
        };

        let mut seen = HashSet::new();

        for (index, line) in manifest.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let (key, value) = line
                .split_once("=")
                .ok_or(error::Error::InvalidManifest)?;

            let key = key.trim();
            let value = value.trim();

            if value.is_empty() {
                return Err(error::Error::InvalidManifest); 
            }

            if !seen.insert(key.to_string()) {
                return Err(error::Error::InvalidManifest);
            }

            match key {
                "name" => result.name = value.to_string(),
                "source" => result.source = value.to_string(),
                "version" => result.version = value.to_string(),
                "checksum" => result.checksum = value.to_string(),
                "deps" => result.deps = Some(value.to_string()),
                "author" => result.author = Some(value.to_string()),
                "description" => result.description = Some(value.to_string()),
                _ => return Err(error::Error::InvalidManifest),
            }
        }
        
        if result.validate_manifest() {
            Ok(result)
        } else {
            return Err(error::Error::InvalidManifest);
        }
    }

}
