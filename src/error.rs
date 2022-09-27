#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown instruction: 0x{0:08x}")]
    UnknownInstruction(u32),
}

pub type Result<T> = std::result::Result<T, Error>;
