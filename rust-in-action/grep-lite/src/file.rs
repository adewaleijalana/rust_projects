#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn open(f: &mut File) -> bool {
    true
}

pub fn close(f: &mut File) -> bool {
    true
}

pub fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}
