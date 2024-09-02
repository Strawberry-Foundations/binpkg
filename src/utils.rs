use std::fs;
use crate::log_fail;

pub fn open_specfile(specfile_path: &str) -> String {
    fs::read_to_string(specfile_path).unwrap_or_else(|_| {
        log_fail!("Invalid specfile");
        std::process::exit(1);
    })
}