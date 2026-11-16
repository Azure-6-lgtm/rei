use crate::error;

pub fn list_packages() -> Result<(), error::Error> {
    println!("rei: listing packages");
    Ok(())
}
