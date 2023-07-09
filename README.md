# Eldiron Cli

[![Latest version](https://img.shields.io/crates/v/eldiron-cli.svg)](https://crates.io/crates/eldiron-cli)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

A command line tool for [`Eldiron`](https://eldiron.com/) RPG creator.

Currently it provides an interactive interface to build your server from scratch, and deploy your server using systemd if it is available on your system.

## Installation

```bash
cargo install eldiron-cli
```

## Usage

Build your server. It will automatically setup a systemd service if it is available:

```bash
eldiron server setup
```

Start the systemd server:

```bash
eldiron server start
```

Restart the systemd server:

```bash
eldiron server restart
```

Stop the systemd server:

```bash
eldiron server stop
```
