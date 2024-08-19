use crate::args::Command;

pub mod commands;
pub mod args;
pub mod colors;
pub mod statics;
pub mod log;

fn main() {
    let args = args::Args::collect();
    let options = args::Args::collect().collect_options();

    match args.command {
        Command::Install => commands::install::install(options.file.unwrap()),
        Command::Extract => commands::extract::extract(options.file.unwrap(), options.destination.unwrap()),
        Command::None => commands::help::help(),
    }
}