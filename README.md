# ops_crdt_rust
Operation based CRDT with VectorClock, Anti-Entropy algorithm and Semi Direct Product

Conflict-free Replicated Data Type (CRDT) replicates supported Data on multiple node
just with async operations. Here we have implementation of operation based CRDT and
it guarantees that all replicated nodes will eventually converge to same value 
(eventual consistency). It makes no assumption about underlying transport mechanism.
There can be network partition, it expects that eventually network partition will 
repaired and nodes will communicate with each other asynchronously.
Messages can be delayed, they can lost, they can reach out-of-order and still it
fullfills its promise of guaranteed eventual consistency.

Here we implement a very small data structure VectorClock and 
Tagged Reliable Causal BroadCast (TRCB).
