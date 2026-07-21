use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("Error: Entropy OS Failure.")]
    OsFailure(#[from] getrandom::Error),

    #[error("Error: Unknown command '{0}'. Use --help for usage.")]
    UnknownCommand(&'a str),

    #[error("Error: Unknown flag '{0}'. Use --help for usage.")]
    UnknownFlag(&'a str),

    #[error("Error: Unexpected positional argument '{0}'. Use --help for usage.")]
    UnexpectedArg(&'a str),

    #[error("Error: Missing password for analyze. Usage: passforge analyze <PASSWORD>")]
    NoPassword,

    #[error(
        "Error: Missing password length for generate_password. Usage: passforge generate_password <LENGTH>"
    )]
    NoPasswordLen,

    #[error("Error: Length must be between {0} and {1} inclusive.")]
    InvalidPasswordLen(usize, usize),

    #[error(
        "Error: Requested password length ({0}) exceeds available character pool ({1} characters). Maximum length with current settings is {1}."
    )]
    MaximumPossibleLenExceeded(usize, usize),

    #[error("Error: No character types selected. At least one character set must be enabled.")]
    NoCharacterSetEnabled,

    #[error(
        "Error: Missing number of words for generate-passphrase. Usage: passforge generate-passphrases <WORDS>"
    )]
    NoPassphraseLen,

    #[error("Error: Number of words must be a positive integer betwen {0} and {1} inclusive.")]
    InvalidPassphraseLen(usize, usize),

    #[error("Error: Invalid Chars.")]
    InvalidChar,
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Error: Parse Error.")]
    ParseError,
    #[error("Error: Invalid Value.")]
    InvalidValue,
}
