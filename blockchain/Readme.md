# Blockchain Programming & Protocol Challenges

This folder contains a comprehensive set of blockchain development and protocol engineering exercises. Each question is phrased as a practical challenge or design prompt, segmented by topic and complexity. Use these to deepen your understanding of blockchain internals, cryptography, consensus, networking, and smart contract systems.

## Key Management & Wallets
1. How do you generate an Ethereum secp256k1 keypair in Rust via a CLI?
2. How can you derive an Ethereum address and compute its EIP-55 checksum?
3. How do you serialize, sign, and broadcast raw Ethereum transactions (EIP-155)?
4. How can you implement BIP-39 mnemonic to key derivation in a CLI?
5. How do you create an encrypted keystore file compatible with geth using Rust?
6. How do you generate a Solana Ed25519 keypair and address via CLI?
7. How can you sign and submit a Solana transfer transaction programmatically?
8. How would you build a multi-chain wallet CLI to manage ETH and SOL keys and sign transactions?
9. How can you implement a hardware-wallet-like signing interface (mocked HSM)?
10. How do you build a replay-protected transaction broadcaster with nonce management?

## Blockchain Data & Indexing
11. How do you fetch the latest Ethereum block and parse transactions using ethers-rs?
12. How can you implement a WebSocket block stream listener for live notifications?
13. How would you build an indexer that stores blocks in RocksDB (light explorer)?
14. How do you decode common Ethereum event topics without an ABI (using heuristics)?
15. How can you write a simple on-chain Solana program (e.g., counter) and client?
16. How do you implement Borsh-based account serialization and upgrade patterns in Solana?
17. How would you build a CLI to display Solana account diffs between slots?
18. How do you implement a local validator test harness for Solana program testing?
19. How can you build an RPC batching client for high-throughput queries?
20. How do you implement an offline transaction signing and broadcast workflow?

## Merkle Trees, State Proofs & Storage
21. How do you implement a Merkle tree with inclusion proofs in Rust?
22. How can you implement a Sparse Merkle Tree and proof verifier?
23. How do you parse the Ethereum state trie (MPT) and traverse accounts?
24. How can you verify an Ethereum account proof against a block state root?
25. How do you implement a light-client header verifier for Ethereum (headers + proofs)?
26. How can you implement Solana account proof verification against a snapshot root?
27. How do you build snapshot export/import for a subset of chain state?
28. How can you implement an append-only transaction log and compaction?
29. How do you build RocksDB-backed ledger storage with column families?
30. How can you implement block-level checkpointing and fast restore?

## Consensus & Protocol Simulation
31. How do you simulate a simplified PoW consensus in Rust?
32. How can you implement a toy PoS validator election and slot clock simulator?
33. How do you implement simplified LMD-GHOST fork-choice and simulate it?
34. How can you build a BLS aggregation demo (toy, not production)?
35. [Protocol] How would you modify a test fork-choice heuristic in a local client simulation?
36. How do you implement a beacon-like proposer/attester duty scheduler (toy)?
37. How can you simulate slashing and penalty rules and their economic effects?
38. How do you implement an epoch/slot timing benchmark harness?
39. How can you compare finality times across toy consensus variants?
40. How do you implement a simplified finality gadget (checkpointing)?

## P2P Networking & Mempool
41. How do you implement Ethereum devp2p handshake and message codec (toy P2P)?
42. How can you implement a discovery v4 node discovery crawler?
43. How do you implement gossiping transaction propagation in a simulated network?
44. How can you build a small peer reputation and scoring service?
45. How do you implement a libp2p-based peer-to-peer overlay for a local testnet?
46. [Protocol] How would you modify propagation rules (gossip delay/topology) and measure sync?
47. How do you implement a block propagation simulator with varying latencies?
48. How can you build a P2P simulator to test eclipse attack vectors?
49. How do you implement a peer blacklisting policy and test it?
50. How can you implement a simple NAT traversal/relay for peers?
51. How do you build a mempool implementation with prioritization and eviction?
52. How can you implement an EIP-1559-like fee market simulator (toy)?
53. How do you implement a local tx pool that orders by tip and gas price (configurable)?
54. [Protocol] How would you alter fee burn vs tip split in a local client and measure miner/validator incentives?
55. How do you implement a mempool scanner and transaction tagging service?
56. How can you implement tx bundle acceptance and reordering logic (builder simulation)?
57. How do you build an RPC-facing mempool metrics exporter (Prometheus)?
58. How can you implement pessimistic vs optimistic tx execution policies?
59. How do you implement visibility timeouts and replay protection in a tx pool?
60. How can you build a mempool snapshot and replay tool?

## Smart Contracts, VM & DeFi Primitives
61. How do you implement a toy smart-contract VM based on WASM (embedding wasmtime)?
62. How can you implement a contract ABI/serialization layer for your WASM VM?
63. How do you build a gas metering layer for the WASM VM (charge per instruction)?
64. How can you implement deterministic state transitions and state root calculation?
65. [Design] How would you propose a new on-chain execution model (WASM-based) and write a spec doc?
66. [Design] How do you implement a prototype of that new execution model (simple VM + state)?
67. How can you design and implement an on-chain upgrade mechanism for the VM (trusted/timelocked)?
68. How do you implement replay protection and transaction reordering rules for the VM?
69. How can you build a test harness for VM fuzzing and gas-bounds testing?
70. How do you evaluate VM performance vs baseline (microbenchmarks)?
71. How can you build a toy AMM (constant product) as an on-chain program on Solana (Rust)?
72. How do you implement an orderbook matching engine (off-chain simulator)?
73. How can you implement simple lending/borrowing logic (collateral + interest)?
74. How do you implement a staking and delegation prototype (on-chain program)?
75. How can you implement an oracle ingestion pipeline and on-chain price feed?
76. [Protocol] How would you design and prototype a protocol-level change for transaction ordering to reduce MEV (spec + sim)?
77. How do you implement a sealed-bid block builder simulation (privacy-first builder)?
78. How can you implement a local flashloan attack and defense playground (simulated)?
79. How do you build an MEV-aware block builder prototype and simulate revenue distribution?
80. How can you compare protocol-level mitigations for MEV in simulation?

## Client Engineering & Protocol Design
81. How do you implement a minimal blockchain client in Rust supporting block import, validation, and local consensus?
82. How can you add P2P sync (headers-first) to your toy client?
83. How do you add state trie/storage to the toy client and support RPC queries?
84. How can you implement light-client sync modes (headers + proofs)?
85. [Protocol] How would you design a change to light-client/headers format (reduce size) and implement sender/receiver in a toy network?
86. How do you implement snapshotting and incremental download (fast sync) for your toy client?
87. How can you add validator set transitions and slashing mechanics?
88. How do you implement client metrics and health endpoints (Prometheus)?
89. How can you build a client integrator that can run multiple consensus rules as feature flags?
90. How do you benchmark sync times across settings and topologies?
91. How can you clone reth (or other Rust Ethereum client) and get it to build and run locally?
92. [Protocol] How would you identify a localizable behavior in reth (e.g., gas calc, mempool order) and prepare a reproducible test?
93. How do you implement and run your modification in reth locally (change, build, test)?
94. How can you submit a patch: add a feature-flagged fork-choice tweak and measure effects?
95. How do you clone Solana validator source, build and run local validator from source?
96. [Protocol] How would you modify Solana validator tick/PoH rate in source and measure block production/latency in a testnet?
97. How can you implement a configurable leader-schedule change in Solana source and test leader distribution effects?
98. How do you implement a consensus parameter change (slot timing, finality threshold) in your toy client and measure safety/liveness tradeoffs?
99. [Design] How would you architect a new protocol layer (spec): e.g., a privacy-preserving block-assembly protocol or alternative finality gadget? Write an RFC and formalize safety properties.
100. [Design & Protocol] How do you implement a minimal end-to-end prototype of your new protocol (consensus + p2p + execution) in Rust, run a multi-node testnet, collect metrics, and write a postmortem comparing it to ETH/SOL baselines?

---

Feel free to contribute solutions, code samples, or improvements!