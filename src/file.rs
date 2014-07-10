use std::io::File;

pub fn read(path: &Path) -> String {
    match File::open(path).unwrap().read_to_string() {
        Err(e) => fail!("Couldn't read {}: {}", path.display(), e),
        Ok(contents) => contents,
    }
}

pub fn write(path: &Path, contents: &str) {
    match File::create(path).write_str(contents) {
        Err(e) => fail!("Couldn't write {}: {}", path.display(), e),
        Ok(_) => {},
    }
}