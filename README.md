# passforge

[![GitHub Actions (CI)](https://img.shields.io/github/actions/workflow/status/JoelD-Ortega/passforge/ci.yml?branch=main)](https://github.com/JoelD-Ortega/passforge/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

> Cryptographically secure password generator focused on security and performance, minimizing dependencies.

A robust command-line tool useful for anyone who values their security and wants to feel protected in their day-to-day life.

## Highlights

- **Security:** Uses `getrandom` for OS-provided entropy and Lemire's algorithm for unbiased random generation.
- **Performance:** Utilizes a `build.rs` script to embed the wordlist at compile time.
- **Safe:** Zero `unsafe` code.
- **Minimalist:** Only two dependencies (`getrandom` and `thiserror`).

## Install

**From crates.io:**


```
sh
cargo install passforge

```


**From source:**


```
sh
git clone https://github.com/JoelD-Ortega/passforge.git
cd passforge
cargo build --release

```


The binary will be located at `target/release/passforge`.

## Usage


```
text
passforge <COMMAND> [OPTIONS] [ARGUMENTS]

```


### Commands

- `analyze <PASSWORD>`: Analyze the entropy of a password.
- `generate-password [LENGTH]`: Generate a random password. Default length is 16.
- `generate-passphrase [WORDS]`: Generate a random passphrase. Default number of words is 6.

### Password Generation Options (for `generate-password`)

- `--no-digits`, `-D`: Disable digits.
- `--no-lowercase`, `-L`: Disable lowercase letters.
- `--no-uppercase`, `-U`: Disable uppercase letters.
- `--no-symbols`, `-S`: Disable symbols.
- `--pin`, `-p`: Generate digits only (PIN). This option overrides any other character set restrictions.

### Passphrase Generation Options (for `generate-passphrase`)

- `-w, --words <N>`: Number of words (default: 6). This can also be provided as a positional argument.

### Examples


```
sh
# Generate a 16-character password (default length)
passforge generate-password

# Generate a 20-character password with no uppercase, digits, or symbols
passforge generate-password 20 -U -D -S

# Generate a 6-digit PIN
passforge generate-password 6 --pin

# Generate a passphrase with 6 words (default)
passforge generate-passphrase

# Generate a passphrase with 8 words
passforge generate-passphrase 8

# Analyze a password's entropy
passforge analyze "MyP@ssw0rd123"

```


## Dependencies

- [`getrandom`](https://crates.io/crates/getrandom): For OS-level cryptographic entropy.
- [`thiserror`](https://crates.io/crates/thiserror): For ergonomic error handling.

## Roadmap

- [ ] **Advanced entropy analysis:** Pattern detection and strength scoring.
- [ ] **Breach detection:** Check if the password has been leaked (Have I Been Pwned API).
- [ ] **Bulk generation:** Generate multiple passwords/passphrases.
- [ ] **Clipboard integration:** Copy output directly to the clipboard.
- [ ] **Configuration file:** Save default options.
- [ ] **TUI mode:** Interactive interface for non-technical users.

## Contributing

We welcome contributions! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## Acknowledgments

- [EFF Large Wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases)
- [Lemire's blog post on unbiased random generation](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/)

## License

[GNU General Public License v3.0 (GPL-3.0)](https://www.gnu.org/licenses/gpl-3.0.en.html)

## Author

Joel D. Ortega - [GitHub Profile](https://github.com/JoelD-Ortega)
