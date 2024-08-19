use libspkg::binpkg::BinPkg;
use libspkg::binpkg::err::BinPkgError;

use crate::colors::{CYAN, RESET};
use crate::{log_fail, log_info};

pub fn extract(file: String, destination: String) {
    let package = match BinPkg::read(&file) {
        Ok(package) => {
            package
        },
        Err(err) => {
            let _: BinPkgError = *err.downcast_ref().unwrap();
            log_fail!(format!("Error while reading package: {err}"));
            std::process::exit(1);
        }
    };

    log_info!(format!("Extracting {CYAN}{file}{RESET} ..."));

    package.self_extract(destination).unwrap();
}