# Step by step tutorial
How to make websockets with Yew `0.21` frontend and Actix backend.

By the end of it, you should grasp how to setup a websocket able to allow direct frontend-backend comunication. The use case we will consider is the frontend starting some long running computation on the backend, which then gradually updates the user on the current state and afterwards updates the user page accordingly.

## Things excluded from this tutorial
In this tutorial we will not cover topics such as authentication, authorization, database access, etc. We will focus on the communication between the frontend and the backend using websockets.

## Prerequisites
Let's get started from the basics. You will need to have installed the following software:

- Rust
- Trunk


Note that, while there are Dockerfiles in this repository, they are not used in this tutorial. They are there to ease reproduction of the environment in which this tutorial was written, but if you follow these steps you will setup your own environment and you won't need to use Docker.

### Installing Rust
First thing first, you need to install Rust. You can do this by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install). This most commonly boils down to running the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing the `wasm32-unknown-unknown` target
Since we will be compiling the Yew frontend to WebAssembly, we need to install the `wasm32-unknown-unknown` target. You can do this by running the following command in your terminal:

```bash
rustup target add wasm32-unknown-unknown
```

### Installing `cargo-watch`
In order to refresh the Actix backend automatically, we will use `cargo-watch`. You can install it by running the following command in your terminal:

```bash
cargo install cargo-watch
```

### Installing Trunk
[Trunk](https://trunkrs.dev/) is a build tool for the Yew frontend. You can install it by running the following command in your terminal:

```bash
cargo install --locked trunk
```


## Creating the crates
We start by creating the frontend and the backend crates, plus the commons crate which will contains struct which we want to use both in the frontend and backend. You can do this by running the following commands in your terminal:

```bash
cargo new backend --bin
cargo new frontend --bin
cargo new commons --lib
```

## Creating the workspace
Since we will be working with multiple crates, we will create a workspace to manage them. You can do this by creating a `Cargo.toml` file in the root directory of the project with the following content:

```bash
touch Cargo.toml
```

```toml
[workspace]
resolver = "2"

members = ["backend", "frontend", "commons"]
```

## We add the commons crate to the backend and frontend
In the `backend` and `frontend` directories, add the following line to the `Cargo.toml` file:

```toml
commons = { path = "../commons" }
```

## Configuring Trunk
In the `frontend` directory, create a file called `Trunk.toml`. We will not actually need to put anything in it for this specific project, but if you would like to see a more complex example of a `Trunk.toml` file, you can look at the [configuration from the emi-monorepo project](https://github.com/earth-metabolome-initiative/emi-monorepo/blob/296c5ec6e03154ad8d960a5a942a428b9415b3fc/web/frontend/Trunk.toml).

```toml
touch frontend/Trunk.toml
```

## Frontend dependencies
In the `frontend` directory, add the following dependencies to the `Cargo.toml` file:

```bash
# Used to log messages to the console of the browser
cargo add wasm-logger
# The standard logging library of Rust
cargo add log
# The frontend framework we will be using
cargo add yew --features csr
```

These should result into a set of dependencies being added to your frontend's `Cargo.toml` file looking somewhat like these:

```toml
TODO! ADD!
```

## Backend dependencies
In the `backend` directory, add the following dependencies to the `Cargo.toml` file:

```bash
# The Actix web framework
cargo add actix-web --features ws
# The logger for the backend
cargo add env_logger
# The standard Rust logger
cargo add log
```

## The index file
We proceed to create a populate the `index.html` file in the `frontend` directory. You can do this by creating a file called `index.html` with the following content:

```bash
touch frontend/index.html
```

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link data-trunk rel="scss" href="styles/index.scss" data-inline/>
    <title>Yew/Actix Websocket tutorial</title>
  </head>
  <body></body>
</html>
```

## SCSS
As you might have already observed from the `index.html` file, we will be using SCSS for styling. We will create a file called `index.scss` in the `frontend/styles` directory, and will gradually add to it as we progress through the tutorial. Trunk will handle the compilation of the SCSS file into CSS, so you don't need to worry about that. You can create the file by running the following commands in your terminal:

```bash
mkdir frontend/styles
touch frontend/styles/index.scss
```

## Websocket messages
The frontend and the backend will exchange messages through the websocket. We will define the messages in the `commons` crate, using an enumeration so to make sure that all possible messages are covered. You can do this by creating a file called `messages.rs` in the `commons/src` directory with the following content:

```bash
touch commons/src/messages.rs
```

```rust
TODO!
```

## Starting the backend
In order to start the backend while making it automatically recompile upon changes, you can run the following command in the `backend` directory:

```bash
cargo watch -x run
```

## Starting the frontend
In order to start the frontend while making it automatically recompile upon changes, you can run the following command in the `frontend` directory:

```bash
trunk serve --port 3000 --proxy-backend http://localhost:8080/ws
```