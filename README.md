# r2

r2 is a CLI tool for creating nested folders/files much like the `mkdir -p` command. Written in Rust, it is cross-platform and can be used on Windows, Linux, and Mac.

> Note: This is a work in progress. I am still working on the CLI and the documentation.
> **essentiallly, this is a glorified mkdir -p and touch**

## Installation

```bash
cargo install --git https://github.com/Karim-W/r2
```

## Usage

### Create Nested Folders

```bash
r2 foo/bar/baz
```

This will create the following folder structure:

```
foo
└── bar
    └── baz
```

### Create a File Within Nested Folders

```bash
r2 foo/bar/baz/file.txt
```

This will create the following folder structure:

```
foo
└── bar
    └── baz
        └── file.txt
```
