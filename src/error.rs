pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotFindRsdp,
    RsdpCheckSumNotZero(u8),
    RsdtCheckSumNotZero(u8),
}
