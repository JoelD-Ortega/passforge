mod error;
mod cli;
mod random;

use error::Error;
use cli::parse;
use cli::Command;
use cli::cmd_analyze;
use cli::cmd_generate_password;
use cli::cmd_generate_passphrase;
use cli::cmd_help;
use cli::cmd_version;

fn run()->Result<(),Error<'static>>{
    let args:Vec<String> = std::env::args().collect();
    match parse(&args){
        Err(error)=>{
            eprintln!("{}",error);
        }
        Ok(command) => match command {
            Command::Analyze{password} => {
                cmd_analyze(password)?;
            }
            Command::GeneratePassword{
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
            Command::GeneratePassphrase{len} => {
                cmd_generate_passphrase(len)?;
            }
            Command::Help => {
                cmd_help();
            }
            Command::Version => {
                cmd_version();
            }
        }
    }
    Ok(())
}

fn main(){
    if let Err(error) = run(){
        eprintln!("{}",error);
    }
}
