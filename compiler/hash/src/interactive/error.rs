//! Hash interactive mode error reporting
//
// All rights reserved 2021 (c) The Hash Language authors

use thiserror::Error;

/// Enum representing the variants of error that can occur when running an interactive session
#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Unkown command `{0}`.")]
    UnrecognisedCommand(String),

    #[error("Command `{0}` does not take any arguments.")]
    ZeroArguments(String),

    // @Future: Maybe provide a second paramater to support multiple argument command
    #[error("Command `{0}` requires one argument.")]
    ArgumentMismatchError(String),

    #[error("Unexpected error: `{0}`")]
    InternalError(String),
}