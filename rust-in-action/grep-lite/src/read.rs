pub trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}
