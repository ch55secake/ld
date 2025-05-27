# lx

An `ls` clone written in Rust, mostly done so that I can learn Rust. 

> [!NOTE]
> This project uses pre-commit-hooks, to make use of them 
> please install `pre-commit`. To set up the hooks run `just setup_precommit`, if `just` is not 
> installed please install it with either `apt` or `brew` depending on your os.

## Building the project 

You can build the project a couple of ways that depending on the tooling that you have installed, assuming you have `cargo`
installed run: 

```bash
cargo clean 
cargo build --workspace
```

If you have `just` installed run, this will run, `clean`, `build` and `check`: 

```bash 
just cbuild
```

### Usage 

> [!NOTE]
> Keep in mind that the binary will most likely be a dev version of the binary depending on how you compiled it, meaning
> you will have to run `./target/dev|release/lx`

Once compiled, you can use it just as you would use `ls` (for the most part) here is the help: 

```bash
List files and directories within a directory.

Usage: lx [OPTIONS] [DIRECTORY]

Arguments:
  [DIRECTORY]  Name of the directory [default: .]

Options:
  -p, --permissions  Show file permissions
  -a, --all          Show all items
  -h, --help         Print help
  -V, --version      Print version
```

## Installing the binary 

There is a couple of options when installing the binary: 

### Build from source and install with cargo

To build from source and install with cargo: 

```bash 
just build_release
cargo install --path . 
```

### Manually move the installed binary

Download the binary from the releases tab on [GitHub](https://github.com/ch55secake/lx) and once installed, move the binary 
with the below commands: 

```bash 
chmod +x ~/Downloads/lx
mv ~/Downloads/lx /usr/local/bin/
```