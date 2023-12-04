# Advent of Code 2023

![GitHub License](https://img.shields.io/github/license/wowkster/advent-of-code-2023?color=blue)
[![Build Status](https://img.shields.io/endpoint.svg?url=https://actions-badge.atrox.dev/wowkster/advent-of-code-2023/badge?ref=main&style=flat)](https://actions-badge.atrox.dev/wowkster/advent-of-code-2023/goto?ref=main)

<p align="center">
    <img src="https://github.com/wowkster/advent-of-code-2023/assets/49880655/39af5b68-66b2-4ef2-9fb1-6678aead065a" width="250">
</p>

This is my implementation of the [Advent of Code](https://adventofcode.com/) programming challenges in Rust.

## Results

| Day                                          | Part 1 | Part 2 |
| -------------------------------------------- | :----: | :----: |
| [Day 1](https://adventofcode.com/2023/day/1) | :star: | :star: |
| [Day 2](https://adventofcode.com/2023/day/2) | :star: | :star: |
| [Day 3](https://adventofcode.com/2023/day/3) | :star: | :star: |
| [Day 4](https://adventofcode.com/2023/day/4) | :star: | :star: |

## Benchmarks

The following benchmarks were created on a Macbook Pro with an M2 Pro processor:

| Day                                                                              | Part 1    | Part 2    |
| -------------------------------------------------------------------------------- | --------- | --------- |
| [Day 1](https://github.com/wowkster/advent-of-code-2023/blob/main/src/bin/01.rs) | `24.2µs`  | `120.8µs` |
| [Day 2](https://github.com/wowkster/advent-of-code-2023/blob/main/src/bin/02.rs) | `40.9µs`  | `40.8µs`  |
| [Day 3](https://github.com/wowkster/advent-of-code-2023/blob/main/src/bin/03.rs) | `260.7µs` | `116.3µs` |
| [Day 4](https://github.com/wowkster/advent-of-code-2023/blob/main/src/bin/04.rs) | `85.2µs` | `104.7µs` |

## Project Structure

This project is organized as a single crate with a different binary for each day and some common helpers to reduce the boilerplate.

- `data` - Holds data that gets run against the code
- `data/examples` - Holds the examples that are provided with the challenges
- `data/examples/xx/part-x.txt` - Since each day has 2 parts, each part is located in a separate file inside a directory for that day. See [Example Format](#example_format).
- `data/inputs/xx.txt` - The input files for each day. See [Downloading Input Files](#downloading_input_files).
- `src/lib.rs` - Contains some helper code to reduce boilerplate such as the `solution!` macro
- `src/main.rs` - Contains a simple binary to download the input files. See [Downloading Input Files](#downloading_input_files).
- `src/bin/xx.rs` - The solution files for each day

## Downloading Input Files

Since input files are unique to every user, you need to supply your session token to be able to download the inputs for your account. This can be done through the `AOC_SESSION_TOKEN` environment variable.

With your session token in your environment, you can invoke the downloader with:

```console
$ cargo run
```

## Running A Specific Day's Solution

To run the solution code for a specific day, all you need to do is supply the correct binary name to cargo:

```console
$ cargo run --bin 01
```

## Running Example Tests

All the solution files include tests for the provided examples in the prompt. See [Example Format](#example_format) for more details. To run the test suite on all the solutions, use:

```console
$ cargo test
```

And to run the tests for a particular solution, use:

```console
$ cargo test --bin 01
```

## Running Benchmarks

All the example tests also include benchmarks to measure the performance of the solutions. You can run the benchmark suite on all the solutions like this:

```console
$ cargo bench
```

Or run them for a specific day like this:

```console
$ cargo bench --bin 01
```

## Example Format

To remove the need for solution files to include tests for the examples, examples for each day are stored in a special format which includes the expected solution, and can easily be parsed by the runner helpers. Example files look like this:

```txt
<expected solution>
---
<example input>
```

For example, Day 1's example for part 1 looks like this:

```txt
142
---
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```
