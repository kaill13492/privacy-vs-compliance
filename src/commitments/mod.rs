use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct Commitment {
    pub hash: [u8; 32],
}

pub fn commit(value: u64, blinding: [u8; 32]) -> Commitment {
    let mut hasher = Sha256::new();
    hasher.update(value.to_le_bytes());
    hasher.update(blinding);

    let result = hasher.finalize();
    Commitment {
        hash: result.into(),
    }
}
