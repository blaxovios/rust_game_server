# Argolysis Studios Game Server

This is a game server built with Rust and the Axum web framework. It is designed to be a simple, lightweight server that can be used to host games.

## Structure

The project is structured as follows:

- `src/`: This is the source code for the server.
  - `handlers/`: This contains the code for the different handlers for the server.
    - `progress_handler.rs`: This is a simple handler that returns a string indicating the progress of the game.
  - `main.rs`: This is the entry point for the server. It sets up the server and starts it listening on port 3000.
  - `utils/`: This contains utility functions for the server.
    - `logger.rs`: This sets up the logger for the server.
- `Cargo.toml`: This is the configuration file for the project. It specifies the dependencies and other configuration options for the project.

## Install and Run

To install and run the server, follow these steps:

1. Install Rust and Cargo from the official Rust installation page.
2. Clone the repository to a directory on your computer.
3. Navigate to the directory in your terminal.
4. Run `cargo run` to build and start the server.
5. Open a web browser and navigate to `http://localhost:3000/` to see the server in action.

## Configure

The server can be configured by modifying the `Cargo.toml` file. For example, you can change the port number that the server listens on by modifying the `axum` dependency:
