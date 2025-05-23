use ream_bls::BLSSignature;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, typenum};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct SyncAggregate {
    pub sync_committee_bits: BitVector<typenum::U512>,
    pub sync_committee_signature: BLSSignature,
}
