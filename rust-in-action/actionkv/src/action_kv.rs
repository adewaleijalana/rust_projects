use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, Seek, SeekFrom},
    path::Path,
};

use crate::key_value_pair::{ByteString, KeyValuePair};

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

    pub fn load(&mut self) -> io::Result<()> {
        let mut file = BufReader::new(&mut self.file);

        loop {
            let position = file.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut file);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, position);
        }

        Ok(())
    }

    pub fn process_record<R: io::Read + Seek>(reader: &mut R) -> io::Result<KeyValuePair> {
        Ok(KeyValuePair { key: (), value: () })
    }
}
