//! Simulating files one step at a time.

use std::fmt::Display;

use rand::{prelude::*, rng};

use crate::read::Read;

fn one_in(denominator: u32) -> bool {
    rng().random_ratio(1, denominator)
}

/// Represents  "FileState",
/// which which can either be `Open` or `Closed`.
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(formatter, "OPEN"),
            FileState::Closed => write!(formatter, "CLOSED"),
        }
    }
}

/// Represents a "file",
/// which probably lives on a file system.

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for File {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} ({})", self.name, self.state)
    }
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    ///
    ///Creates a new, empty `File`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = File::new("f1.txt");
    ///
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's name.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
}

impl Read for File {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn file_test(){
  let mut f3 = File::new("2.txt");

    let mut buffer: Vec<u8> = vec![];

    if f3.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f3 = open(f3).unwrap();
    let f2_length = f3.read(&mut buffer).unwrap();
    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f3);
    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.get_name(), f2_length);
    println!("{}", text)
}
