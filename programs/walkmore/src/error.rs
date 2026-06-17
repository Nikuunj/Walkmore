use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Winner must be one of the challenge participants")]
    InvalidWinner,
    #[msg("Challenge not completed waiting for winner withdraw")]
    ChallengeNotCompleted,
    #[msg("User must be one of the challenge participants")]
    InvalidUser,
    #[msg("Time out, time ended")]
    Timeout,
    #[msg("Time too TimeEarly")]
    TimeEarly,
}
