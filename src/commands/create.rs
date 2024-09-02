use libspkg::binpkg::BinPkg;
use libspkg::binpkg::metadata::Metadata;

use crate::colors::{CYAN, RESET};
use crate::{log_fail, log_ok};
use crate::core::specfile::Specfile;

pub fn create(file: String, destination: String) {
    let specfile = Specfile::from_file(file).package;
    let output_file = format!("{}-{}-{}.binpkg", specfile.id, specfile.version, specfile.architecture);

    match BinPkg::create(
        Metadata::new(
            specfile.name,
            specfile.id,
            specfile.version,
            specfile.description,
            specfile.architecture,
            specfile.author
        ),
        destination,
        output_file.clone()
    ) {
        Ok(..) => {
            log_ok!(format!("Successfully created package as '{CYAN}{output_file}{RESET}'"));
        },
        Err(err) => {
            log_fail!(format!("Error while creating package: {err}"));
        }
    }
}