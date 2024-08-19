use libspkg::binpkg::BinPkg;
use libspkg::binpkg::err::BinPkgError;

use crate::colors::{CYAN, RESET, GREEN};
use crate::{log_fail, log_info};

pub fn info(file: String) {
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

    log_info!(format!("Information about package {CYAN}{file}{RESET}"));
    println!("    {GREEN}Name:{RESET} {}", package.metadata.name);
    println!("    {GREEN}ID:{RESET} {}", package.metadata.id);
    println!("    {GREEN}Version:{RESET} {}", package.metadata.version);
    println!("    {GREEN}Architecture:{RESET} {}", package.metadata.architecture);
    println!("    {GREEN}Author:{RESET} {}", package.metadata.author);
    println!("    {GREEN}Description:{RESET} {}", package.metadata.description);
}