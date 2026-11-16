use crate::error;

pub fn remove_package(package: String) -> Result<(), error::Error> {
    println!("rei: Removing package");
    Ok(())
}
