use anchor_lang::prelude::*;
use light_concurrent_merkle_tree as mt;

declare_id!("3tLM17SwGY5ovjA5Kk7zge4odyAxaMpDdrXqR3XzSY5k");

#[program]
pub mod cerkle {
    use super::*;
}

#[repr(C)]
#[account]
#[derive(Default)]
pub struct CommitmentTree {
    pub version: u8, // helps with keeping track of new trees
    // pub root: Node,
    pub mt: mt::ConcurrentMerkleTree<light_hasher::Keccak, 18>,
    pub bump: u8,
}

impl CommitmentTree {
    pub fn init(&mut self, version: u8, bump: u8) {
        self.version = version;
        self.bump = bump;
    }
}
// -------------------------- Nullifier Tree -------------------------- //

#[repr(C)]
#[account]
#[derive(Debug)]
pub struct NullifierTree {
    pub version: u8, //  helps with keeping track of new trees
    pub mt: mt::MerkleTree,
    pub bump: u8,
}

impl NullifierTree {
    pub fn init(&mut self, version: u8, bump: u8) {
        self.version = version;
        self.bump = bump;
    }
}
