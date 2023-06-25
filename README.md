# SimpleBlockchainRust

# Simple Blockchain Implementation in Rust

This project is a simple implementation of a blockchain using the Rust programming language. It aims to provide a basic understanding of blockchain concepts and demonstrate how they can be implemented in Rust.

## Table of Contents

- [Overview](#overview)
- [Project Structure](#project-structure)
- [Dependencies](#dependencies)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Motivation](#motivation)
- [Contributing](#contributing)
- [License](#license)

## Overview

The goal of this project is to create a simplified blockchain system, where each block contains a set of transactions, and the blocks are linked together using cryptographic hashes. The implementation includes basic data structures, hashing functions, and a server for interacting with the blockchain.

## Project Structure

The project's source code is organized as follows:

- `src/blockchain`: Contains the main blockchain implementation.
  - `block.rs`: Defines the structure and methods for individual blocks in the blockchain.
  - `chain.rs`: Implements the blockchain itself, managing blocks and their validation.
  - `transaction.rs`: Represents a transaction within a block.
  - `mod.rs`: Module file for the blockchain implementation.

- `src/sha`: Contains the SHA hashing function implementation.
  - `sha.rs`: Implements the hashing function used for block validation.
  - `mod.rs`: Module file for the SHA hashing function.

- `src/server`: Contains the server implementation for interacting with the blockchain.
  - `server.rs`: Implements a simple server that exposes endpoints for adding transactions and retrieving the blockchain.
  - `mod.rs`: Module file for the server implementation.

- `src/main.rs`: The entry point of the program.

## Dependencies

This project has the following dependencies:

- [Rust](https://www.rust-lang.org/) (version 1.55 or higher)

## Getting Started

To get started with the project, follow these steps:

1. Clone the repository:


2. Navigate to the project's directory:


3. Build the project using Cargo:


## Usage

To run the project, use the following command:


The server will start running, and you can interact with the blockchain by making requests to the exposed endpoints. For example, you can add transactions or retrieve the current state of the blockchain.

## Motivation

The motivation behind this project was to explore the world of blockchain technology and deepen my understanding of it. As a fan of the Rust programming language, I wanted to leverage its powerful features and expressiveness to build a simple blockchain implementation. By working on this project, I aimed to gain hands-on experience with concepts such as blocks, transactions, hashing, and chain validation. Additionally, this project allowed me to combine my interest in Rust with the fascinating field of blockchain technology.

## Contributing

Contributions to this project are welcome. If you encounter any issues or have suggestions for improvement, please open an issue or submit a pull request.

