# Building

## Hardware Requirements

To [contribute](ceremony.md#contribute), you will need a total of:
- at least 33 GiB of combined RAM and SWAP memory *available*;
- at least 32.1 GiB of disk space for storing the latest valid and the newly computed contributions.

To [check](ceremony.md#check), you will need a total of:
- at least 33 GiB of combined RAM and SWAP memory *available*;
- 16 GiB of disk space to store each contribution to be checked.

If you are using Docker, ensure that sufficient resources are allocated when running built images (see [Linux](https://docs.docker.com/desktop/settings/linux/#resources), [Mac](https://docs.docker.com/desktop/settings/mac/#resources) and [Windows](https://docs.docker.com/desktop/settings/windows/#resources) Docker documentation).

## Get Sources

First, we get the sources:
```shell
git clone https://github.com/zircuit-labs/ceremony.git
```

The project can be built either by using [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) or [`Docker`](https://docs.docker.com/engine/install/).


## Build with Cargo

We build sources using Rust's `cargo`:

```shell
cd ceremony
cargo install --locked --path . --root .
```

Binaries will be available in `./bin`. 

In order to have them directly available in your shell, you can add such path to the PATH environment variable as:

```shell
# Unix 
export PATH=$(pwd)/bin:$PATH

# Windows
set PATH=%CD%\bin;%PATH%
```

## Build with Docker

We build a Docker image using the provided `Dockerfile`:

```shell
cd ceremony
docker build . -t "ceremony"
```

We can then run the built image in interactive mode with
```shell
docker run -it "ceremony" 
```
Binaries will be directly available in the shell: their installation path is automatically added to the `PATH` environment variable.

## Binaries Output

All output is printed using Rust's [log](https://github.com/rust-lang/log) crate. To view such output, the environmental variable `RUST_LOG` needs to be set to (at least) `info` level.

```shell
export RUST_LOG=info
```