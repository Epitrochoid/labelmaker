# Labelmaker

Labelmaker is a utility for naming files created by any commandline utility. It runs the specified 
command and opens a modal to enter a name for the file created by the command. Labelmaker is primarily 
designed to name screenshots taken with ImageMagick's `import` utility.

## Install

With `rust` and `cargo` installed clone this repo then run the following command.

```sh
cargo install --path .
```

The binary `labelmaker` will now be installed in cargo's binary directory.

## Use

Labelmaker takes a command, an output path, and a filename with template strings. The two supported 
template strings are `<name>` and `<timestamp>`. `<name>` will be replaced with the value typed in the 
name entry modal and `<timestamp>` will be replaced with the current date in YYYY-MM-DD format.

### Example

```sh
labelmaker --command import --path ~/screenshots --name "<timestamp>-<name>.png"
```

