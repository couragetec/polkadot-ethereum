
use super::*;

use crate::Config;
use sp_core::H256;
use frame_support::{impl_outer_origin, impl_outer_event, impl_outer_dispatch, parameter_types,
	weights::Weight,
	dispatch::DispatchError
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, IdentifyAccount, Verify}, testing::Header, Perbill, MultiSignature
};
use sp_keyring::AccountKeyring as Keyring;
use sp_std::convert::From;
use frame_system as system;

use artemis_core::{MessageCommitment, MessageDispatch, ChannelId, SourceChannel, SourceChannelConfig, rewards::InstantRewards};
use artemis_ethereum::Log;

mod bridge {
	pub use crate::Event;
	pub use crate::Call;
}

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
    pub enum TestEvent for Test {
		system<T>,
		pallet_balances<T>,
        bridge,
    }
}

pub type Signature = MultiSignature;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type Balance = u128;

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}


parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	/// The ubiquitous event type.
	type Event = TestEvent;
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

// Mock verifier
pub struct MockVerifier;

impl Verifier for MockVerifier {
	fn verify(message: &Message) -> Result<Log, DispatchError> {
		let log: Log = rlp::decode(&message.data).unwrap();
		Ok(log)
	}
}

// Mock Commitments
pub struct MockMessageCommitment;

impl MessageCommitment for MockMessageCommitment {
	fn add(channel_id: ChannelId, target: H160, nonce: u64, payload: &[u8]) -> DispatchResult {
		Ok(())
	}
}

// Mock Dispatch
pub struct MockMessageDispatch;

impl MessageDispatch<(ChannelId, u64)> for MockMessageDispatch {
	fn dispatch(source: H160, id: (ChannelId, u64), payload: &[u8]) {}
}

parameter_types! {
	pub RewardsAccount: AccountId = Keyring::Eve.into();
}

impl Config for Test {
	type Event = TestEvent;
	type Verifier = MockVerifier;
	type MessageCommitment = MockMessageCommitment;
	type MessageDispatch = MockMessageDispatch;
	type RewardsAccount = RewardsAccount;
	type InboundMessageFee = Balance;
	type RewardRelayer = InstantRewards<Self, Balances>;
}

pub type System = system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type Bridge = Module<Test>;


pub fn new_tester() -> sp_io::TestExternalities {
	new_tester_with_config::<Test>(GenesisConfig {
		source_channels: SourceChannelConfig {
			basic: SourceChannel {
				address: H160::zero(),
			},
			incentivized: SourceChannel {
				address: H160::zero(),
			}
		}
	})
}

pub fn new_tester_with_source_channels(basic: H160, incentivized: H160) -> sp_io::TestExternalities {
	new_tester_with_config::<Test>(GenesisConfig {
		source_channels: SourceChannelConfig {
			basic: SourceChannel {
				address: basic,
			},
			incentivized: SourceChannel {
				address: incentivized,
			}
		}
	})
}

pub fn new_tester_with_config<T: Config>(config: GenesisConfig) -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::default().build_storage::<T>().unwrap();

	config.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
