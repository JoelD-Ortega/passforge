# passforge

[![CI](https://img.shields.io/github/actions/workflow/status/JoelD-Ortega/passforge/ci.yml?branch=main&label=CI)](https://github.com/JoelD-Ortega/passforge/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust: 1.70+](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Cryptographically secure password and passphrase generator for the command line, built for security and performance with only two dependencies.

## Summary

passforge is a command-line tool that generates random passwords, PINs, and passphrases, and analyzes the entropy of an existing password. It draws randomness from the operating system and avoids modulo bias, so the output is suitable for real credentials.

## Why

Most people reuse weak passwords because strong ones are tedious to produce. A password generator is only worth using if its randomness can be trusted, so passforge keeps its trust surface small:

- **Secure by default:** uses `getrandom` for OS-provided entropy and Lemire's algorithm for unbiased random generation.
- **Fast:** a `build.rs` script embeds the wordlist at compile time, so nothing is loaded at runtime.
- **Memory safe:** contains zero `unsafe` code.
- **Minimal:** depends only on `getrandom` and `thiserror`.

## Features

- Random passwords with configurable length and character sets.
- PIN generation (digits only).
- Passphrases built from the EFF Large Wordlist, with a configurable word count and separator.
- Entropy analysis of an existing password.

## Project Status

passforge is under active development. The features listed below are planned and not yet implemented:

- [ ] Advanced entropy analysis: pattern detection and strength scoring.
- [ ] Breach detection: check whether a password has leaked, using the Have I Been Pwned API.
- [ ] Bulk generation: generate multiple passwords or passphrases at once.
- [ ] Clipboard integration: copy output directly to the clipboard.
- [ ] Configuration file: save default options.
- [ ] TUI mode: interactive interface for non-technical users.

## Installation

Rust 1.70 or newer is required.

Install from crates.io:

```sh
cargo install passforge
```

Or build from source:

```sh
git clone https://github.com/JoelD-Ortega/passforge.git
cd passforge
cargo build --release
```

The binary is written to `target/release/passforge`.

## Quickstart

Generate a 16-character password:

```sh
passforge generate-password
```

Generate a six-word passphrase:

```sh
passforge generate-passphrase
```

## Usage

```text
passforge <COMMAND> [OPTIONS] [ARGUMENTS]
```

### Commands

| Command | Description |
| --- | --- |
| `analyze <PASSWORD>` | Analyze the entropy of a password. |
| `generate-password [LENGTH]` | Generate a random password. Default length: 16. |
| `generate-passphrase [WORDS]` | Generate a random passphrase. Default number of words: 6. |

### Options for `generate-password`

| Option | Description |
| --- | --- |
| `-D`, `--no-digits` | Disable digits. |
| `-L`, `--no-lowercase` | Disable lowercase letters. |
| `-U`, `--no-uppercase` | Disable uppercase letters. |
| `-S`, `--no-symbols` | Disable symbols. |
| `-p`, `--pin` | Generate digits only (a PIN). Overrides every other character set restriction. |

### Options for `generate-passphrase`

| Option | Description |
| --- | --- |
| `-l`, `--length <N>` | Number of words. Default: 6. Can also be given as a positional argument. |
| `-s`, `--separator <S>` | Separator between words. Default: whitespace. |

### Examples

Generate a 20-character password with no uppercase letters, digits, or symbols:

```sh
passforge generate-password 20 -U -D -S
```

Generate a 6-digit PIN:

```sh
passforge generate-password 6 --pin
```

Generate a passphrase with 8 words:

```sh
passforge generate-passphrase 8
```

Analyze the entropy of a password:

```sh
passforge analyze "MyP@ssw0rd123"
```

## Documentation

- [`getrandom`](https://crates.io/crates/getrandom): OS-level cryptographic entropy.
- [`thiserror`](https://crates.io/crates/thiserror): ergonomic error handling.
- [Lemire's post on nearly divisionless random integer generation](https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/): the algorithm used for unbiased random numbers.
- [EFF Large Wordlist](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases): the source of the passphrase words.

## Limitations

- Bulk generation is not supported yet: each run produces a single password or passphrase.
- Options cannot be saved: there is no configuration file yet, so flags must be passed on every run.
- Output is printed to the terminal only: there is no clipboard integration yet.

## Support

Open an issue in the [issue tracker](https://github.com/JoelD-Ortega/passforge/issues) for bugs, questions, or feature requests.

## Contributing

Contributions are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) for the guidelines.

## Acknowledgments

- The [Electronic Frontier Foundation](https://www.eff.org/) for the large wordlist.
- [Daniel Lemire](https://lemire.me/blog/) for the unbiased random integer generation technique.

## Contact

- **Author:** [Joel D. Ortega](https://github.com/JoelD-Ortega)

## License

Released under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).
