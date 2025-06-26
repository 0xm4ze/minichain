# MiniChain

A simple blockchain implementation in Rust that demonstrates the fundamental concepts of blockchain technology.

## Overview

MiniChain is a minimalist blockchain implementation that includes core blockchain concepts:

- Block creation and mining
- Proof of Work (PoW) consensus mechanism
- Chain validation
- Genesis block generation
- Simple command-line interface

This project serves as an educational tool to understand how blockchains work at a basic level.

## Features

- **Block Structure**: Each block contains index, timestamp, data, previous hash, current hash, and nonce
- **Proof of Work**: Simple mining algorithm with adjustable difficulty
- **Chain Validation**: Verify the integrity of the entire blockchain
- **Interactive CLI**: User-friendly command-line interface to interact with the blockchain

## Technical Implementation

- Written in Rust for performance and memory safety
- Uses SHA-256 for cryptographic hashing
- JSON serialization support
- Command-line interface for interaction

## Getting Started

### Prerequisites

- Rust and Cargo installed [from the official site](https://www.rust-lang.org/tools/install)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/minichain.git
cd minichain
cargo build --release
```

### Running MiniChain

```bash
cargo run
```

## Usage

The CLI offers four options:

1. **Add Block**: Add a new block with custom data to the blockchain
2. **Show Chain**: Display all blocks in the current blockchain
3. **Validate Chain**: Check if the blockchain is valid
4. **Exit**: Quit the application

## Project Structure

- `src/main.rs`: Entry point and CLI implementation
- `src/block.rs`: Block structure and mining logic
- `src/blockchain.rs`: Blockchain management and validation

## Dependencies

- `chrono`: For timestamp handling
- `serde` and `serde_json`: For serialization
- `sha2`: For SHA-256 hashing

## Future Improvements

- Persistent storage
- Networking capabilities for distributed nodes
- Transaction system
- Advanced consensus mechanisms
- Web interface

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgements

This project was created as a learning exercise to better understand blockchain technology fundamentals.
