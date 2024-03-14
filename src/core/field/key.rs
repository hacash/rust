

// ChannelId ***********************

pub type ChannelId = Fixed16;
pub const CHANNEL_ID_SIZE: usize = ChannelId::width();

// Lockbls

pub type LockblsId = Fixed18;
pub const LOCKBLS_ID_SIZE: usize = LockblsId::width();

// lending

pub type DiamondSyslendId = Fixed14;
pub type BitcoinSyslendId = Fixed15;
pub type UserLendingId = Fixed17;

pub const DIAMOND_SYSLEND_ID_SIZE: usize = DiamondSyslendId::width();
pub const BITCOIN_SYSLEND_ID_SIZE: usize = BitcoinSyslendId::width();
pub const USER_LENDING_ID_SIZE:    usize = UserLendingId::width();
