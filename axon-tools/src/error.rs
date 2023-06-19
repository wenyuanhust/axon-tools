use alloc::string::{String, ToString};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    InvalidProofBlockHash,
    NotEnoughSignatures,
    VerifyMptProof,

    #[cfg(feature = "hex")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "hex")))]
    Hex(faster_hex::Error),

    #[cfg(feature = "proof")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
    Bls(blst::BLST_ERROR),

    #[cfg(feature = "proof")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
    Trie(cita_trie::TrieError),
}

#[cfg(feature = "hex")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hex")))]
impl From<faster_hex::Error> for Error {
    fn from(value: faster_hex::Error) -> Self {
        Self::Hex(value)
    }
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl From<blst::BLST_ERROR> for Error {
    fn from(e: blst::BLST_ERROR) -> Self {
        Self::Bls(e)
    }
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl From<cita_trie::TrieError> for Error {
    fn from(e: cita_trie::TrieError) -> Self {
        Self::Trie(e)
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidProofBlockHash => "Invalid proof block hash".to_string(),
            Error::NotEnoughSignatures => "Not enough signatures".to_string(),
            Error::VerifyMptProof => "Verify mpt proof".to_string(),
            #[cfg(feature = "hex")]
            Error::Hex(e) => alloc::format!("Hex error: {:?}", e),
            #[cfg(feature = "proof")]
            Error::Bls(e) => alloc::format!("Bls error: {:?}", e),
            #[cfg(feature = "proof")]
            Error::Trie(e) => alloc::format!("Trie error: {:?}", e),
        }
    }
}
