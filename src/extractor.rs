use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Extractor {
    dll_path: String,
    valid_data: Vec<u8>,
}

impl Extractor {
    pub fn new<P: AsRef<Path>>(dll_path: P) -> Self {
        Self {
            dll_path: dll_path.as_ref().to_string_lossy().into_owned(),
            valid_data: Vec::new(),
        }
    }

    pub fn process(&mut self) -> Result<(), String> {
        self.extract_data_pattern()
    }

    pub fn get_valid_data(&self) -> &[u8] {
        &self.valid_data
    }

    fn extract_data_pattern(&mut self) -> Result<(), String> {
        let mut f = File::open(&self.dll_path).map_err(|e| format!("File not found: {}", e))?;
        let mut dll_data = Vec::new();
        f.read_to_end(&mut dll_data).map_err(|e| e.to_string())?;

        let head_pattern = b"C\0F\0G\0\0\0\0\0";
        let tail_pattern = b"\0\0\0\0";

        let head_pos = dll_data
            .windows(head_pattern.len())
            .position(|window| window == head_pattern)
            .ok_or("Head pattern not found")?;

        let start_pos = head_pos + head_pattern.len();

        let tail_pos = dll_data[start_pos..]
            .windows(tail_pattern.len())
            .position(|window| window == tail_pattern)
            .map(|pos| pos + start_pos)
            .ok_or("Tail pattern not found")?;

        self.valid_data = dll_data[start_pos..tail_pos].to_vec();
        Ok(())
    }
}
