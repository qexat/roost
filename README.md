# Roost

> Do you speak baguette? Read the [fronch version](./README_FR.md)!

Roost is a Rust error generator written in ~~Python~~ Rust. It is meant to be used for jokes.

It's a simple script I've written in like 2 hours so don't expect the code quality to be good 😆.

## Usage

### Build

```
cargo build --release
```

### Run

In the terminal emulator, type:

```
./target/release/roost
```

Then, fill the fields ; it will print the error message at the end.

### `--output` option

You can write the error message (with the ANSI escape sequences) into a file.

To do so, you simply need to provide a valid file path after the argument, such as:

```
./target/release/roost --output /path/to/my_file.txt
```

## Screenshot

![example.png](./images/example.png)
