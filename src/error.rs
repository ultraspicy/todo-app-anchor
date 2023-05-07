use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Already marked.")]
    AlreadyMarked,
}
