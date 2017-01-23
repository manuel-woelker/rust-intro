use std::fs::remove_file;
use std::io::Error;
fn main() {
    let mut foo2 = vec![1, 2, 3];
    let foo3 = &mut foo2;
    let foo4 = &foo2;
}

pub fn open_file() -> Result<(), Error> {
    remove_file("/file")?;
    Ok(())
}