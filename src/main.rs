/*
    Building a simple merkle tree structure, starting with append-only

    - using SHA-256 hash fucntion
    - key will be u64
    - leaves will be hash of key
    - Hash will be represented as 32-byte fixed-size hash value.  
        - 256 = 32 * 8, 32 bytes
        - keys will be u64, so 8 * 8 bytes.  
        - Hashing a key will be: Hash = SHA256(8 bytes of key)
        - will result in a 32-byte hash value.
    

    type Hash = 32-byte fix-sized hash value

    fn hash_key(key: u64) -> Hash
        - returns Hash = H(serialized_key)

    fn hash_internal(left: Hash, right: Hash) -> Hash
        - concatenates left and right (left || right)
        - returns Hash = H(left || right)


    Will store nodes added to the tree inside a vector of vectors: levels = Vec<Vec<Hash>>;
        - leaves will always be at levels[0] because the tree grows upward.  In regular Merkle trees, you are not concerned
        with the position in the tree based on the binary path.  Leaves determine structure based solely on their order
        - For example:
            key1 = 4
            key2 = 8
            key3 = 10
        levels[0] = [ H(4), H(8), H(10) ]

            - Here, you would compute the hash H( H(4) || H(8) ) to get parent 1 and H( H(10) || H(10) ) to get parent 2.
            - Then the two parents would be hashed together to get the root node


    Design:

    struct MerkleTree {
        levels: Vec<Vec<Hash>>
    }

    Core methods:
    - new() -> MerkleTree
        - Creates an empty tree
        - levels starts empty, no leaves or root.

    - append(key: u64)
        - hashes key using hash_key(key) to get a leaf hash
        - appends leaf hash to levels[0], or create levels[0] if it doesn't exist
        - rebuilding upper levels by
            - hashing together leaves in pairs, duplicating the last leaf if the length of that level is odd.
            - using hash_internal method

    - root() -> Option<Hash>
        - returns the root hash if it exists, otherwise returns none. 

*/

///Hash function
use sha2::{Digest, Sha256};

/// A 32-byte hash value (e.g. SHA-256 output).
pub type Hash = [u8; 32];

type Key = u64;


/*
    Helper functions
*/

/// Hash a u64 key into a 32-byte Hash.
/// (Implementation to be filled in later.)
fn hash_key(key: Key) -> Hash {

    //Construct a hasher
    let mut hasher = Sha256::new();

    //convert key into bytes.  Big Endian
    let key_bytes = key.to_be_bytes();

    //Hash key_bytes
    hasher.update(&key_bytes);

    let result = hasher.finalize();

    //Convert GenericArray<u8, 32> into [u8; 32]
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);

    hash
}

/// Hash two child hashes into their parent hash.
/// (Implementation to be filled in later.)
fn hash_internal(left: Hash, right: Hash) -> Hash {

    //Construct a hasher
    let mut hasher = Sha256::new();

    //add left hash to hasher
    hasher.update(&left);

    //add right hash to hasher, this will concatenate with left hash to yield (left || right)
    hasher.update(&right);

    //finalize
    let result = hasher.finalize();

    //Convert GenericArray<u8, 32> into [u8; 32]
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);

    hash
}

fn hash_to_hex(hash: &Hash) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}


/*
    MerkleTree structure
*/

/// An append-only Merkle tree storing levels of hashes.
/// - levels[0] = leaf level
/// - levels[last] = root level (single hash) when non-empty
pub struct MerkleTree {
    levels: Vec<Vec<Hash>>,
}

impl MerkleTree {
    /// Creates an empty Merkle tree.
    pub fn new() -> Self {
        MerkleTree { 
            levels: Vec::new(),
        }
    }

    /// Appends a new key as a leaf and rebuilds upper levels.
    pub fn append(&mut self, key: Key) {
        //hash key
        let leaf = hash_key(key);

        //Check if there is a leaf level, then push the leaf
        if self.levels.is_empty(){
            //if empty, create the leaf level with this single leaf
            self.levels.push(vec![leaf]);
        }
        else{
            //if leaf level exists, push to leaf level
            self.levels[0].push(leaf);
        }

        //start recomputing the parent hashes, starting at level 1
        let mut level_index = 1;

        loop{
            // Get the level below (the one we just updated or created)
            let below = &self.levels[level_index - 1];

            // If the level below has only one node, it's already the root.
            // No need to build further levels.
            if below.len() == 1 {
                // Truncate any old levels above this (in case they existed).
                self.levels.truncate(level_index);
                break;
            }

            // Build the next level from `below` by hashing pairs
            let mut next_level: Vec<Hash> = Vec::new();

            let mut i = 0;
            while i < below.len() {
                let left = below[i];

                // If there is a right sibling, use it; otherwise duplicate left.
                let right = if i + 1 < below.len() {
                    below[i + 1]
                } else {
                    left
                };

                let parent = hash_internal(left, right);
                next_level.push(parent);

                i += 2;
            }

            // Now insert or replace this next level in self.levels
            if self.levels.len() > level_index {
                // Replace existing level
                self.levels[level_index] = next_level;
            } else {
                // Push as a new level
                self.levels.push(next_level);
            }

            // Move up one level
            level_index += 1;
        }
    }



    /// Returns the current root hash, or None if the tree is empty.
    pub fn root(&self) -> Option<Hash> {
        // If there are no levels, the tree is empty → no root
        let last_level = self.levels.last()?;

        // If last level is empty (shouldn't happen, but safe to check)
        if last_level.is_empty() {
            return None;
        }

        // Return the single hash in the top level
        Some(last_level[0])
    }

}



fn main() {
    
    let mut tree = MerkleTree::new();
    tree.append(5);
    tree.append(10);

    println!("Root: {}", hash_to_hex(&tree.root().unwrap()));

    tree.append(30);

    println!("New root: {}", hash_to_hex(&tree.root().unwrap()));

}
