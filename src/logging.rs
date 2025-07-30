use std::fs::File;
use std::io::{Write, Read};
use sha2::{Sha256, Digest};

// Stuct for logging data in a secure manner
pub struct Logger
{
    logging_directory: String,
    logging_itteration: usize,
    log_line_current: usize,
    log_line_limit: usize,
}
impl Logger
{
    pub fn new(logging_directory: String, line_limit: usize) -> Self
    {
        let log_directory = logging_directory.strip_suffix("\\");
        match log_directory
        {
            Some(directory) => {
                return Logger
                {
                    logging_directory: directory.to_string(),
                    logging_itteration: 0,
                    log_line_current: 0,
                    log_line_limit: line_limit
                };
            }, None => {
                return Logger
                {
                    logging_directory: logging_directory,
                    logging_itteration: 0,
                    log_line_current: 0,
                    log_line_limit: line_limit
                }
            }
        }
    }
    pub fn log_string(&mut self, data_to_log: String) -> bool
    {
        let mut last_hash: Option<Vec<u8>> = None;
        if self.log_line_current >= self.log_line_limit
        {
            let old_file_path: String = format!("{}\\AV_Log_{:?}.log", self.logging_directory, self.logging_itteration);
            self.logging_itteration += 1;
            let mut hasher: Sha256 = Sha256::new();
            let mut file_bytes: Vec<u8> = Vec::new();
            let mut file_handle = File::open(old_file_path).unwrap();
            _ = file_handle.read_to_end(&mut file_bytes);
            hasher.update(file_bytes);
            last_hash = Some(hasher.finalize().to_vec());
        }
        let log_path: String = format!("{}\\AV_Log_{:?}.log", self.logging_directory, self.logging_itteration);
        let file_result = File::create(log_path);
        if file_result.is_err()
        {
            return false;
        }
        let mut file_handle: File = file_result.unwrap();
        if last_hash.is_some()
        {
            _ = file_handle.write_all(&last_hash.unwrap());
            _ = file_handle.write_all("\n".as_bytes());
        }
        let line_to_write: Vec<u8> = format!("{}\n", data_to_log).into_bytes();
        _ = file_handle.write_all(&line_to_write);
        self.log_line_current += 1;
        return true;
    }
}