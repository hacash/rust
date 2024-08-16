# Hacash by rust
Hacash Fullnode, PoWorker etc.

### Build and use documentation

- [Build_compilation.md](https://github.com/hacash/doc/blob/main/build/build_compilation.md)
- [Configuration description](https://github.com/hacash/doc/blob/main/build/config_description.md)


### Project code engineering architecture

The architecture of Hacash full node code is divided into 7 levels from bottom to top:

X16RS -> Core -> Chain -> Mint -> Node -> Server -> Miner

Each layer of the architecture has independent functions and responsibilities for the upper layer to call, and the implementation of the lower layer to the upper layer is unknown. The responsibilities of each layer are roughly as follows:

1. [X16RS] Basic algorithm - including HAC mining, block diamond mining, GPU version algorithm, etc.
2. [Core] Core - block structure definition, interface definition, data serialization and deserialization, storage object, field format, genesis block definition, etc.
3. [Chain] Chain - underlying database, block and transaction storage, blockchain state storage, logs, etc.
4. [Mint] Mint - block mining difficulty adjustment algorithm, coinbase definition, block construction, transaction execution and status update, etc.
5. [Node] Node - P2P underlying module, Backend blockchain synchronization terminal, point-to-point network message definition and processing, etc.
6. [Server] Server - RPC API interface service, block and transaction and account data query, other services, etc.
7. [Miner] Miner - block construction and mining, diamond mining, transaction memory pool, mining pool server, mining pool worker, etc.

