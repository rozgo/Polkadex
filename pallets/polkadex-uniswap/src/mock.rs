use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill};
use frame_system as system;



impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

parameter_types! {
    pub const TradingPathLimit: usize = 10;
}


impl Trait for Test {
    type Event = ();
    type TradingPathLimit = TradingPathLimit;
}

parameter_types! {
    pub const MaxSubAccounts: u32 = 10;
    pub const MaxRegistrars: u32 = 10;
}

impl pallet_idenity::Trait for Test {
    type Event = ();
    type MaxSubAccounts = MaxSubAccounts;
    type MaxRegistrars= MaxRegistrars;


}
parameter_types! {
    pub const MaxLocks: u32 = 10;
    pub const ExistentialDeposit: u128 = 10;
}

impl polkadex_custom_assets::Trait for Test{
    type Event = ();
    type Balance = u128;
    type MaxLocks = MaxLocks;
    type ExistentialDeposit = ExistentialDeposit;
}

pub type PolkadexSwapEngine = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

