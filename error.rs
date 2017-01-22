use std::fs::remove_file;
use std::io::Error;
fn main() {

}

pub fn open_file() -> Result<(), Error> {
    remove_file("/file")?;
    Ok(())
}