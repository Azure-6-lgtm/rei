use crate::error;
use crate::manifest::Manifest;
use std::fs;

pub fn check_manifest(raw_manifest_path: String) -> Result<(), error::Error> {
    println!("rei: Checking manifest");
    let raw_manifest = match fs::read_to_string(raw_manifest_path) {
        Ok(something) => something,
        Err(e) => return Ok(()),
    };

     match Manifest::parse_manifest(&raw_manifest) {
        Ok(_manifest) => {
            println!("rei: Valid Manifest");
            Ok(())
        }
        Err(e) => {
            println!("rei: Invalid Manifest");
            Err(e)
        }
    }
}
