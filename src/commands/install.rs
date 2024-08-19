use libspkg::binpkg::BinPkg;
use libspkg::binpkg::err::BinPkgError;
use crate::commands::extract::extract;
use crate::{log_fail, log_info};

pub fn install(file: String) {
    karen::escalate_if_needed().unwrap();
    
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

    extract(file.clone(), "/tmp/_binpkg.workdir".to_string());

    log_info!(format!("Installing {} version {}", package.metadata.id, package.metadata.version));
    
    subprocess::Exec::shell("cp -r /tmp/_binpkg.workdir/* /").popen().unwrap();
    subprocess::Exec::shell("rm -r /tmp/_binpkg.workdir").popen().unwrap();

}