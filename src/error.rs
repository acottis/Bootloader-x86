pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotFindRsdp,
    RsdpCheckSumNotZero,
    RsdtCheckSumNotZero,

    /// Either not implemented or failing to parse packet from
    /// network buffer
    CouldNotParsePacket,
}
