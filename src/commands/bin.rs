use crate::error;

pub fn get_binary(package: String) -> Result<(), error::Error> {
    println!("rei: Fetching and installing binary of package");
    Ok(())
}
