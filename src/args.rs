use std::env;
use crate::commands;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Command {
    Install,
    Info,
    Extract,
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
    pub file: Option<String>,
    pub destination: Option<String>,
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
            "info" => args.command = Command::Info,
            "extract" => args.command = Command::Extract,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            file: None,
            destination: None
        };


        // let iter = self.args.clone().into_iter().skip(1);
        let args: Vec<String> = self.args.clone().into_iter().skip(1).collect();

        match self.command {
            Command::Install => {
                if args.len() != 1 {
                    commands::help::help();
                    std::process::exit(1);
                }

                options.file = Some(args[0].clone());
            }
            Command::Info => {
                if args.len() != 1 {
                    commands::help::help();
                    std::process::exit(1);
                }

                options.file = Some(args[0].clone());
            }
            Command::Extract => {
                if args.len() != 2 {
                    commands::help::help();
                    std::process::exit(1);
                }

                options.file = Some(args[0].clone());
                options.destination = Some(args[1].clone())
            }
            _ => { }
        }


        options
    }


}