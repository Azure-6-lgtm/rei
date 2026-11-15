use crate::error::Error;

pub fn get(package: String) {
    get_package(package);
}

fn get_package(package: String) -> Result<(), Error> {
    println!("rei: Fetching and Building package");
    Ok(())
}
