use crate::error;

pub struct Manifest {
    name: String,
    source: String,
    version: String,
    deps: Option<String> 
}
