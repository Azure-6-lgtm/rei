use crate::error::Error;

pub fn get_package(package: String) -> Result<(), Error> {
    println!("rei: Fetching and Building package");
    Ok(())
}
