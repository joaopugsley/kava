# KAVA

## Overview
**KAVA** is a [Redis](https://redis.io/) clone implemented in ``Rust``. It aims to ``replicate`` the ``basic functionality`` of ``Redis``, focusing on ``simplicity`` and ``performance``.

## Features
* **Key-Value Store**: Store and retrieve data using a simple key-value interface.
* **Networking**: Communicate with ``KAVA`` using the [Redis Serialization Protocol (RESP)](https://redis.io/docs/latest/develop/reference/protocol-spec/) over TCP/IP.

## Installation

⚠️ If you do not have **cargo**, install using [Rust's Installation Documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)

Clone the repository:
```bash
$ git clone https://github.com/joaopugsley/kava.git
```
Run the project:
```bash
# development
$ cargo run

# build
$ cargo build
```

### Usage:
Connect to the server using a ``Redis Client``. For example, using [redis-cli](https://redis.io/downloads/):
```bash
$ redis-cli -h 127.0.0.1 -p 6379

# Try basic commands. For example, to test the connection and server response, you can use the PING command:
$ PING "kava is awesome"

# This should return:
$ PONG "kava is awesome"
```

## License

The ``KAVA`` source files are distributed under the MIT License found in the LICENSE file.