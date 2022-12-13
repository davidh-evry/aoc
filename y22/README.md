# Advent of code solutions 2022

## Input files
Input files must be placed in a directory named `res` next to the `src` directory and have the format `day[x].txt`. Like this:
```text
├── Cargo.toml
├── res
│   ├── day1.txt
│   ...
│   └── day25.txt
└── src
    └── bin
        ├── day1.rs
        ...
        └── day25.rs
```
## How to run
Run with cargo like this
```sh
cargo run --bin day$X.rs
```
Where `$X` is the day you wish to run, eg. for day 1:
```sh
cargo run --bin day1.rs
```
