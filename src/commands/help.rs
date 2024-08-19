use crate::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};
use crate::statics::VERSION;

pub fn help() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}binpkg v{VERSION}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}binpkg {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}help:{C_RESET} Prints this message
    {CYAN}{BOLD}install <pkg>:{C_RESET} Installs a binpkg
    {CYAN}{BOLD}extract <pkg> <dst>:{C_RESET} Extracts a binpkg file to a specific destination
");
    std::process::exit(0);
}