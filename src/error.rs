use thiserror::Error;

#[derive(Error, Debug)]
pub enum MavecError {
    #[error("Json structure parse error: {0}")]
    JsonStructureParseError(String),
}
