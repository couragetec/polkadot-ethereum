// Mock runtime

use crate::{Module, GenesisConfig, Config};
use sp_core::{H160, H256};
use frame_support::{
	impl_outer_origin, impl_outer_event, impl_outer_dispatch, parameter_types,
	weights::Weight,
	dispatch::DispatchResult,
};
use sp_runtime::{
	traits::{
		BlakeTwo256, IdentityLookup, IdentifyAccount, Verify,
	}, testing::Header, Perbill, MultiSignature,
	ModuleId,
};
use frame_system as system;

use artemis_core::{ChannelId, SubmitOutbound};

use crate as dot_app;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {
		artemis_dispatch
	}
}

impl_outer_dispatch! {
	pub enum Call for Test where origin: Origin {
			frame_system::System,
			pallet_balances::Balances,
			artemis_dispatch::Dispatch,
			dot_app::DOT,
	}
}

impl_outer_event! {
	pub enum Event for Test {
			system<T>,
			pallet_balances<T>,
			artemis_dispatch<T>,
			dot_app<T>,
	}
}

pub type Signature = MultiSignature;

pub type Balance = u128;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

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
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
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

impl artemis_dispatch::Config for Test {
	type Origin = Origin;
	type Event = Event;
	type MessageId = u64;
	type Call = Call;
	type CallFilter = ();
}

pub struct MockSubmitOutbound;

impl SubmitOutbound for MockSubmitOutbound {
	fn submit(_: ChannelId, _: H160, _: &[u8]) -> DispatchResult {
		Ok(())
	}
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}


parameter_types! {
	pub const DotModuleId: ModuleId = ModuleId(*b"s/dotapp");
}

impl Config for Test {
	type Event = Event;
	type Currency = Balances;
	type SubmitOutbound = MockSubmitOutbound;
	type CallOrigin = artemis_dispatch::EnsureEthereumAccount;
	type ModuleId = DotModuleId;
}

pub type System = system::Module<Test>;
pub type Dispatch = artemis_dispatch::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type DOT = Module<Test>;

pub fn new_tester() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let config: GenesisConfig = GenesisConfig {
		address: H160::repeat_byte(1),
	};
	config.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
