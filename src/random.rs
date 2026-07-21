include!(concat!(env!("OUT_DIR"), "/wordlist_data.rs"));

use crate::error::Error;
use std::collections::HashSet;
const DIGITS_SET: &[u8; 10] = br#"0123456789"#;
const LOWERCASE_SET: &[u8; 26] = br#"abcdefghijklmnopqrstuvwxyz"#;
const UPPERCASE_SET: &[u8; 26] = br#"ABCDEFGHIJKLMNOPQRSTUVWXYZ"#;
const SYMBOLS_SET: &[u8; 32] = br##"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##;
pub const DIGITS_SET_SIZE: usize = 10;
pub const LOWERCASE_SET_SIZE: usize = 26;
pub const UPPERCASE_SET_SIZE: usize = 26;
pub const SYMBOLS_SET_SIZE: usize = 32;
const WORDLIST_SIZE: usize = 7776;

#[inline]
fn get_random() -> Result<u32, Error<'static>> {
    let mut buffer = [0u8; 4];
    getrandom::fill(&mut buffer)?;
    Ok(u32::from_ne_bytes(buffer))
}

fn get_random_in_range(limit: usize) -> Result<usize, Error<'static>> {
    let threshold: usize = (0b11111111111111111111111111111111 - limit + 1) % limit;
    loop {
        let random_num: usize = get_random()? as usize;
        let m: u64 = (random_num as u64) * (limit as u64);
        let low: usize = m as usize;
        if low >= threshold {
            return Ok((m >> 32) as usize);
        }
    }
}

pub fn generate_random_password(
    password_len: usize,
    digits: bool,
    lowercase: bool,
    uppercase: bool,
    symbols: bool,
    charset_size: usize,
    reps: bool,
) -> Result<String, Error<'static>> {
    let mut password: String = String::with_capacity(charset_size);
    let mut set: HashSet<u8> = HashSet::new();
    let mut character: u8;
    let mut charset: Vec<u8> = Vec::with_capacity(charset_size);
    if digits {
        charset.extend_from_slice(DIGITS_SET);
    }
    if lowercase {
        charset.extend_from_slice(LOWERCASE_SET);
    }
    if uppercase {
        charset.extend_from_slice(UPPERCASE_SET);
    }
    if symbols {
        charset.extend_from_slice(SYMBOLS_SET);
    }

    for _ in 0..password_len {
        loop {
            character = charset[get_random_in_range(charset_size)?];
            if set.insert(character) || reps {
                break;
            }
        }
        password.push(character as char);
    }
    Ok(password)
}

#[inline]
fn get_random_word() -> Result<&'static str, Error<'static>> {
    Ok(WORDS[get_random_in_range(WORDLIST_SIZE)?])
}

pub fn get_random_passphrase(wordlist: &mut Vec<&str>) -> Result<(), Error<'static>> {
    let mut set: HashSet<&str> = HashSet::new();
    let mut word: &str;
    for item in wordlist.iter_mut() {
        loop {
            word = get_random_word()?;
            if set.insert(word) {
                *item = word;
                break;
            }
        }
    }
    Ok(())
}
