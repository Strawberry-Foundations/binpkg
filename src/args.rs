use std::env;
use crate::colors::{BOLD, C_RESET, RED, RESET};

pub enum Command {
    Install,
    None
}

pub struct Args {
    pub args: Vec<String>,
    pub command: Command,
    pub command_str: String,
    pub options: Options,
}

#[derive(Default)]
pub struct Options {
    pub file: String,
}

impl Args {
    pub fn collect() -> Self {
        let mut args = Self {
            args: vec![],
            command: Command::None,
            command_str: String::new(),
            options: Options { ..Default::default() }
        };

        let collector: Vec<String> = env::args().collect();

        if collector.len() <= 1 {
            return args
        }

        let parser: Vec<String> = env::args().skip(1).collect();

        args.args.clone_from(&parser);
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "install" => args.command = Command::Install,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            file: String::default(),
        };


        let mut iter = self.args.clone().into_iter().skip(1);

        while let Some(arg) = iter.next() {
            let _ = arg.as_str();
            if let Some(val) = iter.next() {
                if let Ok(file) = val.parse::<String>() {
                    options.file = file
                } else {
                    eprintln!("{RED}{BOLD} ! {RESET} Invalid file{C_RESET}");
                }
            } else {
                eprintln!("{RED}{BOLD} ! {RESET} Missing file{C_RESET}");
            }
        }

        options
    }


}