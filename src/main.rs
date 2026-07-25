mod cli;
mod error;
mod random;

use cli::Command;
use cli::cmd_analyze;
use cli::cmd_generate_passphrase;
use cli::cmd_generate_password;
use cli::cmd_help;
use cli::cmd_version;
use cli::parse;
use error::Error;

fn run() -> Result<(), Error<'static>> {
    let args: Vec<String> = std::env::args().collect();
    match parse(&args) {
        Err(error) => {
            eprintln!("{}", error);
        }
        Ok(command) => match command {
            Command::Analyze { password } => {
                cmd_analyze(password)?;
            }
            Command::GeneratePassword {
                len,
                charset_size,
                digits,
                lowercase,
                uppercase,
                symbols,
                reps,
            } => {
                cmd_generate_password(
                    len,
                    digits,
                    lowercase,
                    uppercase,
                    symbols,
                    charset_size,
                    reps,
                )?;
            }
            Command::GeneratePassphrase { len, separator } => {
                cmd_generate_passphrase(len, separator)?;
            }
            Command::Help => {
                cmd_help();
            }
            Command::Version => {
                cmd_version();
            }
        },
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
    }
}
