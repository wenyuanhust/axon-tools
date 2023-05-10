use keccak_hasher::KeccakHasher;
use trie_db::Hasher;

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
    KeccakHasher::hash(data)
}
