---
sidebar_position: 1
title: Quickstart
---

# Quickstart

This guide will help you get started with `snapctl`.

## Download

You can download the latest release of `snapctl` from the [GitHub Releases page](https://github.com/open-sori/snapctl/releases).

## Build from Source

To build `snapctl` from source, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

1. Clone the repository:

    ```bash
    git clone https://github.com/open-sori/snapctl.git
    cd snapctl
    ```

2. Build the project in release mode:

    ```bash
    cargo build --release
    ```

## Docker Image

You can also use the `snapctl` Docker image.

### Manualy build the image

```bash
docker build --build-arg SNAPCTL_VERSION=v1.0.0 --build-arg SNAPCTL_ARCH=aarch64-unknown-linux-musl --build-arg CREATED_DATE=$(date +%Y-%m-%d) -t snapctl .
docker run --rm snapctl --version
```

## Usage

After building, you can run `snapctl` from the `target/release` directory:

```bash
./target/release/snapctl --host 127.0.0.1 --port 1780 <COMMAND>
```

Alternatively, during development, you can use `cargo run`:

```bash
cargo run -- --host 127.0.0.1 --port 1780 set client salon --volume 100
```

Replace `127.0.0.1` and `1780` with your Snapcast server's IP address and port.

### Examples

**Get all clients:**

```bash
./target/release/snapctl get clients
```

**Set a client's volume:**

```bash
./target/release/snapctl set client --client-id <CLIENT_ID> --volume 80
```

**Mute a group:**

```bash
./target/release/snapctl set group --group-id <GROUP_ID> --mute true
```

**Delete a client:**

```bash
./target/release/snapctl delete client --client-id <CLIENT_ID>
```

**Display the version:**

```bash
./target/release/snapctl version
```