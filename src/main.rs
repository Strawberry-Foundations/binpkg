use crate::args::Command;

pub mod commands;
pub mod args;
pub mod colors;
mod statics;

fn main() {
    let args = args::Args::collect();
    let options = args::Args::collect().collect_options();

    match args.command {
        Command::Install => {

        },
        Command::None => commands::help::help(),
    }
    println!("Hello, world!");
}
