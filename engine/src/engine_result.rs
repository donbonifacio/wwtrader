use error::EngineError;

pub type EngineResult<T> = std::result::Result<T, EngineError>;
