#[derive(Clone, Debug)]
pub enum Error {
    #[cfg(feature = "proof")]
    InvalidProofBlockHash,

    #[cfg(feature = "proof")]
    NotEnoughSignatures,

    #[cfg(feature = "proof")]
    Bls(blst::BLST_ERROR),
}

#[cfg(feature = "proof")]
impl From<blst::BLST_ERROR> for Error {
    fn from(e: blst::BLST_ERROR) -> Self {
        Self::Bls(e)
    }
}
