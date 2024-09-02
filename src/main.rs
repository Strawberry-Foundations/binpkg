use core::args;
use core::args::Command;

pub mod commands;
pub mod colors;
pub mod statics;
pub mod core;
pub mod utils;

fn main() {
    let args = args::Args::collect();
    let options = args::Args::collect().collect_options();

    match args.command {
        Command::Install => commands::install::install(options.file.unwrap()),
        Command::Info => commands::info::info(options.file.unwrap()),
        Command::Extract => commands::extract::extract(options.file.unwrap(), options.destination.unwrap()),
        Command::Create => commands::create::create(options.file.unwrap(), options.destination.unwrap()),
        Command::None => commands::help::help(),
    }
}