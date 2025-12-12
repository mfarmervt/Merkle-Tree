## Merkle Tree 

A simple, append-only Merkle tree implementation with Rust using SHA-256.  

An educational project built to explore a cryptographic data structure involving hashing and a tree representation using a 2D vector.  

## Features:

- Append-only Merkle tree structure  
- `u64` keys stored as leaf hashes  
- SHA-256 hashing for leaves and internal nodes  
- Duplicate the last node in an odd-lengthed leaf vector to compute parent hashes 
- Automatic recomputation of parent levels on each append  
- Simple API:
  - `new()`
  - `append(key)`
  - `root() -> Option<Hash>`

 ## How it works

 - Each u64 key is converted into 8 bytes and hashed using SHA-256
 - Internal parent nodes are computed by hashing the concatenation of two child hashes
 - This is repeated moving up the tree from the leaf nodes and stops when a non-leaf level of the tree has only one element (the root).
