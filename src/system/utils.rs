use std::{fs::File, io::Read, path::Path};

pub fn read_file(path: &Path) -> Result<String, ()> {
    let mut string = String::new();
    let mut file = match File::open(path) {
        Ok(the_file) => the_file,
        Err(_) => return Err(()),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => return Ok(string),
        Err(_) => return Err(()),
    }
}
