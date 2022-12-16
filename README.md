# Advent of Code 2022

My solutions for the [Advent of Code 2022](https://adventofcode.com/2022) challenges written in Rust.  
My goal is to try and write the solutions using idiomatic code. Also I try to make the solutions not panic on any invalid input.

## Project structure

Inputs need to be placed in the `inputs` directory. The input for the challenge of day `n` needs to be placed in the file `inputs/day_n.txt` where n is always `0` padded 2 digits (e.g. 01, 02, 03...).

## Usage

```console
Advent of Code 2022

Usage: advent-of-code-2022 [OPTIONS] <DAY> [PART]

Arguments:
  <DAY>   Challenge day
  [PART]  Day part [default: 1]

Options:
  -s, --std-input  Flag to take input from standard input instead of file, useful for the small examples or piping in custom input
  -h, --help       Print help information
  -V, --version    Print version information
```

## Build

```console
cargo build --release
```
