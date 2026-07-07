use std::{
    collections::HashMap, fs::{File, OpenOptions}, io::{self, BufReader, BufWriter, Read, Seek, SeekFrom}, path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::key_value_pair::{ByteStr, ByteString, KeyValuePair};

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

    pub fn process_record<R: io::Read>(reader: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = reader.read_u32::<LittleEndian>()?;
        let key_len = reader.read_u32::<LittleEndian>()?;
        let val_len = reader.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            reader
                .by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }

        debug_assert_eq!(data.len(), data_len as usize);

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;

        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64>{
        let file = BufWriter::new(&mut self.file);
        let key_len = key.len();
        let value_len = value.len();

        let mut temp = ByteString::with_capacity(key_len + value_len);

        for byte in key {
            temp.push(*byte);
        }

        for byte in  value{
            temp.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp);

        let next_byte = SeekFrom::End(0);

        let current_position = file.seek(SeekFrom::Current(0))?;
        file.seek(next_byte)?;
        file.write_u32::<LittleEndian>(checksum)?;
        file.write_u32::<LittleEndian>(key_len as u32)?;
        file.write_u32::<LittleEndian>(value_len as u32)?;

        f.write_all(&mut tmp)?;

        Ok(current_position)

    }
}
