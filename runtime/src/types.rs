use sp_runtime::{
	generic,
	traits::{IdentifyAccount, Verify},
	MultiSignature,
};

pub use module_primitives::{AirDropCurrencyId, Amount, Balance, CurrencyId, EraIndex};
pub use module_support::{ExchangeRate, Price, Rate, Ratio};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

pub type AuctionId = u32;

pub type Share = u128;

pub type Moment = u64;
