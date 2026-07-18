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
        Ok(command)=>{
            match command{
                Command::Analyze(command)=>{
                    cmd_analyze(command.password)?;
                }
                Command::GeneratePassword(command)=>{
                    cmd_generate_password(command.len,command.digits,command.lowercase,command.uppercase,command.symbols,command.charset_size,command.reps)?;
                }
                Command::GeneratePassphrase(command)=>{
                    cmd_generate_passphrase(command.len)?;
                }
                Command::Help=>{
                    cmd_help();
                }
                Command::Version=>{
                    cmd_version();
                }
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
