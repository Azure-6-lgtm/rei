use crate::error;

pub fn info_package(package: String) -> Result<(), error::Error> {
    println!("rei: Info of package");
    Ok(())
}
