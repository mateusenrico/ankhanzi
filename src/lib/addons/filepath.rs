use std::{fs::File, path::Path};

pub trait RdFile {
    fn rdfile(path: &str) -> File {
        File::open(Path::new(path)).unwrap()
    }
}

impl RdFile for File {}
