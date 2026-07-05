use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self},
    path::Path,
};

use crate::key_value_pair::ByteString;

#[derive(Debug)]
pub struct ActionKV {
    file: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        let index = HashMap::new();
        Ok(ActionKV { file, index })
    }
}
