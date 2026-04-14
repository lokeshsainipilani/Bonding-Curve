use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Unauthorized address")]
    UnauthorizedAddress,

    #[msg("Amount Out is smaller than required amount")]
    InsufficientAmountOut,

    #[msg("InSufficient funds")]
    InsufficientFunds,

    #[msg("Curve limit reached")]
    CurveLimitReached,

    #[msg("Value is not in expected range")]
    IncorrectValueRange,

    #[msg("Incorrect fee recipient")]
    IncorrectFeeRecipient,

    #[msg("An overflow or underflow occured during calculation")]
    InvalidReserves,

    #[msg("Curve is not initialised")]
    CurveNotInitialized,

    #[msg("Curve is not completed")]
    CurveNotCompleted,

    #[msg("Already migrated to Raydium")]
    AlreadyMigrated,

    #[msg("Mathmatical operation overflow")]
    MathOverflow,

    #[msg("Insufficient SOL balance")]
    InSufficientSolBalance,

    #[msg("Insufficient token balance")]
    InufficientTokenBalance,

    #[msg("Invaid pool owner")]
    InvalidPoolOwner,

    #[msg("Invalid pool state")]
    InvalidPoolState,

    #[msg("Invalid pool tokens")]
    InvalidPoolTokens,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Division by zero not allowed")]
    DivisionByZero,

    #[msg("Invalid token allocation - must allocate at least 80% to bonding curve")]
    InvaldTokenAllocation,

    #[msg("Invalid curve limit - must be exactly 42 SOL")]
    InvalidCurveLimit,

    #[msg("Invlaid initial Sol reserve - must be exactly 12.33 SOL")]
    InvalidInitialSolReserve
    
}