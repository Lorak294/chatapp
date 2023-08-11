# Chatapp
Simple console chat server project made with Rust. 

## Overview
This project was made to learn some basics about Rust programming language and review parallel programming using TCP sockets.

## Usage
Chatapp is built entirely as a single Rust workspace, so build is pretty straightforward:

```bash
# Clone the repo
git clone https://github.com/Lorak294/chatapp.git

# Build everything in the workspace
cd chatapp
cargo build --release
```

The workspace includes one shared library and two applications: client and server. First run the server:

```bash
# run the server
cargo run -p server -r
```

In other terminal run the client and connect to the server by typing the server address:
```bash
# run the client
cargo run -p client -r

# inisde the application you wil be requested for the server address,you can type ":local"
# to use the hardcoded localhost:8080 for both server and client:
:local
```

After successfully connecting, every line you write inside the client application will be
sent to the server and forwarded to every currently connected client, including the sender. Due to sending serialized
custom Message enums (created to make message display coherent) using other applications (eg. Telnet) as clients 
may not work.

To end the client connection, inside the client application type `:quit` or end the proccess with `CTRL + C`.
To shutdown the whole server just end the server proccess with `CTRL + C`.

## Used stack
-   **Language:** [Rust](https://www.rust-lang.org/es)
-   **Libs:** [Tokio](https://tokio.rs/), [Serde](https://serde.rs/)
-   **Protocols:** [TCP](https://www.rfc-editor.org/rfc/rfc793.html)
