//! The same enum with every code distinct.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum EscrowError {
    InvestorBatchEmpty = 70,
    InvestorBatchTooLarge = 71,
    InvestorNotAllowlisted = 104,
    ComputePayoutArithmeticOverflow = 129,
    ProtocolFeeBpsOutOfRange = 215,
    YieldTierTableInvalid = 236,
    FeesLimitOutOfRange = 237,
}
