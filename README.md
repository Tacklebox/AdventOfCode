# Advent Rust Workspace

Fork this repo to have a quick and convenient framework for participating in the
advent of code.

## Basic Setup

```sh
git clone $YOUR_FORK
cd advent_rust_workspace
cargo advent auth --session-cookie $YOUR_ADVENT_SESSION_COOKIE
```

## How to set up an advent binary

```sh
cargo advent new --year 2018 --day 1 # It will ask automatically if you don't
                                     # provide these arguments
cd 2018/1
# fill in the part1() function in src/main.rs
cargo advent submit
# fill in the part2() function in src/main.rs
cargo advent submit --part 2
```

## Other features/tips

If you `cargo add tracing` in your solution for one of the days, you can use
`tracing::debug!("whatver your message is");` to print debug messages to stderr.
These are only visible if you pass `--debug` to the binary like
`cargo run -- --part 1 --debug`. Functionality to collect and display debug
information with `cargo advent run -vv` will be added soon.
