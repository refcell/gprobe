//! [go-ethereum](https://github.com/ethereum/go-ethereum) Table Prefixes

/// Header prefix + num (uint64 big endian) + hash -> header
pub const HEADER_PREFIX: &[u8] = b"h";

/// Header Prefix + num (uint64 big endian) + hash + Header TD Suffix -> td
pub const HEADER_TD_SUFFIX: &[u8] = b"t";

/// Header Prefix + num (uint64 big endian) + hash + Header Hash Suffix -> hash
pub const HEADER_NUMBER_PREFIX: &[u8] = b"H";

/// Block Body Prefix + num (uint64 big endian) + hash -> block body
pub const BLOCK_BODY_PREFIX: &[u8] = b"b";

/// Block Receipts Prefix + num (uint64 big endian) + hash -> block receipts
pub const BLOCK_RECEIPTS_PREFIX: &[u8] = b"r";

/// Transaction Lookup Prefix + hash -> transaction/receipt lookup metadata
pub const TX_LOOKUP_PREFIX: &[u8] = b"l";

/// Bloom Bits Prefix + bit (uint16 big endian) + section (uint64 big endian) + hash -> bloom bits
pub const BLOOM_BITS_PREFIX: &[u8] = b"B";

/// Snapshot Account Prefix + account hash -> account trie value
pub const SNAPSHOT_ACCOUNT_PREFIX: &[u8] = b"a";

/// Snapshot Storage Prefix + account hash + storage hash -> storage trie value
pub const SNAPSHOT_STORAGE_PREFIX: &[u8] = b"o";

/// Code Prefix + code hash -> account code
pub const CODE_PREFIX: &[u8] = b"c";

/// Skeleton Header Prefix + num (uint64 big endian) -> header
pub const SKELETON_HEADER_PREFIX: &[u8] = b"S";
