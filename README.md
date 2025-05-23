# rust-blockchain

Rust blockchain is my uni project to dive into blockchain tech while leveling up my low-level Rust game.  
It’s a CLI tool that decentralizes block generation, validation, and creation. Clients connect manually via gRPC since automatic network discovery would be too ambitious for the 3-week deadline.

## Components

The rust blockchain is built around a few essential pieces:

- **Block generation**  
  Blocks are generated with proper hashing, timestamps, and cryptographic links to the previous block. Each block carries the basic data needed to secure the chain’s integrity.

- **Blockchain management**  
  Local chain handling, in-memory storage, block validation, and appending new blocks.

- **Peer-to-peer communication**  
  Nodes sync manually over gRPC, sharing blocks and peer lists—no auto-discovery, all manual join moves.

- **Simple consensus**  
  The longest valid chain wins, preventing forks and keeping all nodes on the same page.

## Usage

- Start a Rust CLI node:  
  ```bash
  cargo run -- --port 5000
