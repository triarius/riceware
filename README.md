Riceware
===

Yet another passphrase generator in a long line of similarly named passphrase generators. This time, it's in Rust, so it will be blazingly fast™️

# Installation
```shell
cargo install --git https://github.com/triarius/riceware
```

# Usage
```
Usage: riceware [OPTIONS]

Options:
  -n, --num-words <NUM_WORDS>  The number of words in the passphrase [default: 4]
  -s, --separator <SEPARATOR>  The string to separate words in the passphrase [default: " "]
  -d, --dict-path <DICT_PATH>  A path to a dictionary file. A builtin dictionary is used if not provided
  -h, --help                   Print help
  -V, --version                Print version
```
