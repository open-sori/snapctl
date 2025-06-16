# SNAPCTL

A command-line utility for controlling and monitoring Snapcast servers and clients.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
  - [Get Commands](#get-commands)
  - [Set Commands](#set-commands)
  - [Delete Commands](#delete-commands)
  - [Version Command](#version-command)
- [Configuration](#configuration)
- [Environment Variables](#environment-variables)
- [Examples](#examples)
- [Testing](#testing)
- [Building from Source](#building-from-source)
- [Building the Binary](#building-the-binary)
- [Building the Container Image](#Building-the-container-image)

## Features

- Control Snapcast servers and clients via command line
- Get information about servers, clients, streams, and groups
- Modify client and group settings
- Delete clients
- JSON-RPC over WebSocket communication with Snapcast server

## Installation

### Pre-built Binaries

Download the appropriate binary for your platform from the [releases page](https://github.com/your-repo/releases).

### Using Cargo

```bash
cargo install snapctl
```

## Usage

```bash
snapctl [OPTIONS] <COMMAND>
```

## Commands

### Get Commands

Get information about various Snapcast components:

- `get streams`: Get information about all streams
- `get stream <STREAM_ID>`: Get information about a specific stream
- `get groups`: Get information about all groups
- `get group <IDENTIFIER>`: Get information about a specific group
- `get clients`: Get information about all clients
- `get client <CLIENT_ID>`: Get information about a specific client

### Set Commands

Modify Snapcast client and group settings:

- `set client`: Set client properties
  - `--client-id`: The ID of the client to modify
  - `--mute`: Whether to mute the client (true/false)
  - `--volume`: Volume percentage to set (0-100)
  - `--latency`: Latency in milliseconds to set
  - `--name`: Name to set for the client
  - `--group`: Group ID to assign the client to

- `set group`: Set group properties
  - `--group-id`: The ID of the group to modify
  - `--name`: Name to set for the group
  - `--mute`: Whether to mute the group (true/false)
  - `--stream-id`: Stream ID to set for the group
  - `--clients`: Comma-separated list of client IDs to assign to the group

### Delete Commands

Delete Snapcast resources:

- `delete client <CLIENT_ID>`: Delete a client
- `delete clients <CLIENT_IDS>`: Delete multiple clients (comma-separated list)

### Version Command

Display the version of the `snapctl` utility:

- `version`: Show the current version of the application.

## Configuration

The utility can be configured using a YAML configuration file located at `~/.config/snapctl/config.yaml`.

## Environment Variables

You can set the following environment variables:

- `SNAPSERVER_HOST`: Default host address (default: "127.0.0.1")
- `SNAPSERVER_PORT`: Default port number (default: 1780)

## Examples

Get information about all clients:

```bash
snapctl get clients
```

Set a client's volume:

```bash
snapctl set client --client-id myclient --volume 80
```

Mute a group:

```bash
snapctl set group --group-id mygroup --mute true
```

Delete a client:

```bash
snapctl delete client --client-id oldclient
```

Using a custom host and port:

```bash
snapctl -H 192.168.1.100 -p 1781 get streams
```

Display the version:

```bash
snapctl version
```

## Testing

To test the application during development, you can use the following command:

```bash
cargo run -- get clients
```

This command will:

1. Compile the application in development mode
2. Run the binary with the `get clients` command
3. Connect to the default Snapcast server (127.0.0.1:1780)
4. Retrieve and display information about all connected clients

You can test other commands similarly:

```bash
cargo run -- get server
cargo run -- set client --client-id myclient --volume 75
```

## Building from Source

1. Clone the repository:

```bash
git clone https://github.com/your-repo/snapctl.git
cd snapctl
```

2. Install dependencies:

```bash
cargo build
```

## Building the Binary

To build an optimized release binary:

```bash
cargo build --release
```

The binary will be available at `target/release/snapctl` (or `target/release/snapctl.exe` on Windows).

For a production build, you might want to:

1. Clean any previous builds:

```bash
cargo clean
```

2. Build with optimizations:

```bash
cargo build --release
```

3. The final binary will be in the `target/release` directory.

## Building the Container Image

### Manualy build the image

```bash
docker build --build-arg CREATED_DATE="$(date +'%Y-%m-%d')" --build-arg SNAPCTL_VERSION="v1.0.0" --build-arg TARGETARCH="arm64" -t snapctl .
docker run --rm snapctl --version
export GHCR_PAT="YOUR_TOKEN"
echo $GHCR_PAT | docker login ghcr.io -u tdesaules --password-stdin
docker image tag snapctl:latest ghcr.io/open-sori/snapctl:v1.0.0
docker push ghcr.io/open-sori/snapctl:v1.0.0
docker pull ghcr.io/open-sori/snapctl:v1.0.0
```
