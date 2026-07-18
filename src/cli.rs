include!(concat!(env!("OUT_DIR"),"/help.rs"));
include!(concat!(env!("OUT_DIR"),"/version.rs"));

use crate::error::Error;
use crate::error::InternalError;
use crate::random::generate_random_password;
use crate::random::get_random_passphrase;
use crate::random::DIGITS_SET_SIZE;
use crate::random::LOWERCASE_SET_SIZE;
use crate::random::UPPERCASE_SET_SIZE;
use crate::random::SYMBOLS_SET_SIZE;

const MIN_PASSWORD_LEN:usize = 4;
const MAX_PASSWORD_LEN:usize = 1024;
const DEFAULT_PASSWORD_LEN:usize = 16;
const MAX_PASSPHRASE_LEN:usize = 48;
const DEFAULT_PASSPHRASE_LEN:usize = 6;

/*
struct Opt{
    name:&'static str,
    description:&'static str,
}

const OPTIONS: &[OPT]=&[
    Opt{ name:"--analyze", description:""},
    Opt{ name:"--generate-password", description:""},
    Opt{ name:"--generate-passphrase", description:""},
    Opt{ name:"--help", description:""},
    Opt{ name:"--version", description:""},
];
*/

pub struct GeneratePasswordCommand{
    pub len:usize,
    pub digits:bool,
    pub lowercase:bool,
    pub uppercase:bool,
    pub symbols:bool,
    pub charset_size:usize,
    pub reps:bool,
}

pub struct AnalyzeCommand<'a>{
    pub password:&'a str,
}

pub struct GeneratePassphraseCommand{
    pub len:usize,
}

pub enum Command<'a>{
    Analyze(AnalyzeCommand<'a>),
    GeneratePassword(GeneratePasswordCommand),
    GeneratePassphrase(GeneratePassphraseCommand),
    Help,
    Version,
}

fn parse_num(num:&str,min:usize,max:usize)->Result<usize,InternalError>{
    let mut digits:usize = 0;
    for &byte in num.as_bytes(){
        if byte < b'0' || byte > b'9'{
           return Err(InternalError::ParseError);
        }
        else if byte != b'0'{
            digits += 1;
        }
    }
    if digits > 0 && digits < 5{
        let mut parsed_num:usize = 0;
        for byte in num.as_bytes(){
            parsed_num = parsed_num * 10 + (byte - b'0') as usize;
        }
        if parsed_num >= min && parsed_num <= max{
            return Ok(parsed_num);
        }
    }
    Err(InternalError::InvalidValue)
}

fn parse_command_analyze<'a>(args:&'a[String])->Result<Command<'a>,Error<'a>>{
    let mut iter = args.iter();
    let mut password:&str = "";
    let mut parsed_password:bool = false;
    while let Some(arg) = iter.next(){
        match arg.as_str(){
            "--password" | "-p" =>{
                password = iter.next().ok_or(Error::NoPassword).unwrap();
                parsed_password = true;
            }
            _=>{
                if parsed_password{ return Err(Error::UnknownCommand(arg));}
                else{
                    password = arg;
                    parsed_password = true;
                }
            }
        }
    }
    Ok(Command::Analyze(AnalyzeCommand{password}))
}

fn parse_command_generate_password<'a>(args:&'a[String])->Result<Command<'a>,Error<'a>>{
    let mut password_len:usize = DEFAULT_PASSWORD_LEN;
    let mut charset_size:usize = DIGITS_SET_SIZE + LOWERCASE_SET_SIZE + UPPERCASE_SET_SIZE + SYMBOLS_SET_SIZE;
    let mut digits:bool = true;
    let mut lowercase:bool = true;
    let mut uppercase:bool = true;
    let mut symbols:bool = true;
    let mut reps:bool = false;
    let mut parsed_password_len = false;
    let mut iter = args.iter();

    while let Some(arg) = iter.next(){
        match arg.as_str(){
            "--no-digits" | "-D" =>{
                if digits {charset_size -= DIGITS_SET_SIZE;}
                digits = false;
            }
            "--no-lowercase" | "-L" =>{
                if lowercase {charset_size -= LOWERCASE_SET_SIZE;}
                lowercase = false;
            }
            "--no-uppercase" | "-U" =>{
                if uppercase {charset_size -= UPPERCASE_SET_SIZE;}
                uppercase = false;
            }
            "--no-symbols" | "-S" =>{
                if symbols {charset_size -= SYMBOLS_SET_SIZE;}
                symbols = false;
            }
            "--pin" | "-p" =>{
                lowercase = false;
                uppercase = false;
                symbols = false;
                charset_size = DIGITS_SET_SIZE;
            }
            "--length" | "-l" =>{
                let temp_len:&str = iter.next().ok_or(Error::NoPasswordLen).unwrap();
                password_len = parse_num(temp_len,MIN_PASSWORD_LEN,MAX_PASSWORD_LEN).map_err(|_| Error::InvalidPasswordLen(MIN_PASSWORD_LEN,MAX_PASSWORD_LEN))?;
                parsed_password_len = true;
            }
            "--repetitions" | "-r" => {reps = true;}
            _ =>{
                if parsed_password_len{
                    return Err(Error::UnknownCommand(arg));
                }
                else{
                    password_len = parse_num(arg,MIN_PASSWORD_LEN,MAX_PASSWORD_LEN).map_err(|_| Error::InvalidPasswordLen(MIN_PASSWORD_LEN,MAX_PASSWORD_LEN))?;
                    parsed_password_len = true;
                }
            }
        }
    }
    if !digits && !lowercase && !uppercase && !symbols{
        return Err(Error::NoCharacterSetEnabled);
    }
    if charset_size < password_len && !reps{
        return Err(Error::MaximumPossibleLenExceeded(password_len,charset_size));
    }
    Ok(Command::GeneratePassword(GeneratePasswordCommand{len:password_len,digits,lowercase,uppercase,symbols,charset_size,reps}))
}

fn parse_command_generate_passphrase<'a>(args:&'a[String])->Result<Command<'a>,Error<'a>>{
    let mut passphrase_len:usize = DEFAULT_PASSPHRASE_LEN;
    let mut _separator:u8 = b' ';
    let mut parsed_len:bool = false;
    let mut iter = args.iter();
    while let Some(arg) = iter.next(){
        match arg.as_str(){
            "--separator" | "-s" => {
                //separator = iter.next().ok_or(Error::NoCharacterSetEnabled)?

            }
            "--length" | "-l" => {
                let temp_len = iter.next().ok_or(Error::NoPassphraseLen).unwrap();
                passphrase_len = parse_num(temp_len,MIN_PASSWORD_LEN,MAX_PASSWORD_LEN).map_err(|_| Error::InvalidPassphraseLen(1,MAX_PASSPHRASE_LEN))?;
                parsed_len = true;
            }
            _ => {
                if parsed_len{
                    return Err(Error::UnknownCommand(arg));
                }
                else{
                    passphrase_len = parse_num(arg,MIN_PASSWORD_LEN,MAX_PASSWORD_LEN).map_err(|_| Error::InvalidPassphraseLen(1,MAX_PASSPHRASE_LEN))?;
                    parsed_len = true;
                }
            }
        }
    }
    Ok(Command::GeneratePassphrase(GeneratePassphraseCommand{len:passphrase_len}))
}

pub fn parse<'a>(args:&'a[String])->Result<Command<'a>,Error<'a>>{
    match args[1].as_str(){
        "analyze"=>{
            Ok(parse_command_analyze(&args[2..])?)
        }
        
        "generate-password"=>{
            Ok(parse_command_generate_password(&args[2..])?)
        }
        
        "generate-passphrase"=>{
            Ok(parse_command_generate_passphrase(&args[2..])?)
        }
        
        "--help" | "help" | "h"=>{
            Ok(Command::Help)
        }

        "--version" | "version" | "-V"=>{
            Ok(Command::Version)
        }
        
        _=>{
            Err(Error::UnknownCommand(args[1].as_str()))
        }
    }
}

fn analyze_password(password:&str)->Result<u32,Error<'static>>{
    let mut bitmask:u32=0;
    for c in password.chars(){
        if (c as usize) < 33 || (c as usize) > 126{
            return Err(Error::InvalidChar);
        }
        else if c.is_ascii_uppercase() && bitmask & 1 << 0 == 0{
            bitmask |= 1<<0;
        }
        else if c.is_ascii_lowercase() && bitmask & 1 << 1 == 0{
            bitmask |= 1<<1;
        }
        else if c.is_ascii_digit() && bitmask & 1 << 2 == 0{
            bitmask |= 1<<2;
        }
        else if bitmask & 1 << 3 == 0{
            bitmask |= 1<<3;
        }
        else{
            break;
        }
    }
    Ok(bitmask)
}

#[inline]
pub fn cmd_analyze(password:&str)->Result<(),Error<'static>>{
    let res:u32=analyze_password(password)?;
    let mut set_size:u32=0;
    if res&(1<<0)==1<<0{
        set_size+=26;
    }
    if res&(1<<1)==1<<1{
        set_size+=26;
    }
    if res&(1<<2)==1<<2{
        set_size+=10;
    }
    if res&(1<<3)==1<<3{
        set_size+=32;
    }
    let entropy:f32=(password.len() as f32) * (f32::log2(set_size as f32));
    println!("Password:{}\nEntropy:{} bits\n",password,entropy);
    Ok(())
}

#[inline]
pub fn cmd_generate_password(password_len:usize,digits:bool,lowercase:bool,uppercase:bool,symbols:bool,charset_size:usize,reps:bool)->Result<(),Error<'static>>{
    let password:String=generate_random_password(password_len,digits,lowercase,uppercase,symbols,charset_size,reps)?;
    println!("{}",password);
    Ok(())
}

#[inline]
pub fn cmd_generate_passphrase(passphrase_len:usize)->Result<(),Error<'static>>{
    let mut wordlist:Vec<&str> =vec!["";passphrase_len];
    get_random_passphrase(&mut wordlist)?;
    println!("{}",wordlist.join(" "));
    Ok(())
}

#[inline]
pub fn cmd_version(){
    println!("{}",VERSION);
}

#[inline]
pub fn cmd_help(){
    println!("{}",HELP);
}
