use crate::{Module, Trait};

use sp_io::TestExternalities;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill, Perquintill, FixedPointNumber,
};

use frame_system as system;
use frame_support::{
    impl_outer_origin, impl_outer_dispatch, parameter_types,
    weights::{Weight, IdentityFee},
    dispatch::{DispatchResult},
};

use pallet_profile_follows::Call as ProfileFollowsCall;
use frame_support::traits::Currency;
pub use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};

// TODO: replace with imported constants from Runtime
pub const SMNS: Balance = 210_000_000_000;
pub const DOLLARS: Balance = SMNS;             // 1_000_000_000_000
pub const CENTS: Balance = DOLLARS / 100;      // 10_000_000_000
pub const MILLICENTS: Balance = CENTS / 1_000; // 10_000_000

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_dispatch! {
	pub enum Call for Test where origin: Origin {
		frame_system::System,
		pallet_balances::Balances,
		pallet_profile_follows::ProfileFollows,
	}
}

#[derive(Clone, Eq, PartialEq)]
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
    type Call = Call;
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
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;
parameter_types! {
    pub const ExistentialDeposit: u64 = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Trait for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

// TODO export to a common place
parameter_types! {
  pub const IpfsCidLen: u32 = 46;
  pub const MinHandleLen: u32 = 5;
  pub const MaxHandleLen: u32 = 50;
}

impl pallet_utils::Trait for Test {
    type Event = ();
    type Currency = Balances;
    type MinHandleLen = MinHandleLen;
    type MaxHandleLen = MaxHandleLen;
}

impl pallet_profile_follows::Trait for Test {
    type Event = ();
    type BeforeAccountFollowed = ();
    type BeforeAccountUnfollowed = ();
}

parameter_types! {}

impl pallet_profiles::Trait for Test {
    type Event = ();
    type AfterProfileUpdated = ();
}

// TODO export to a common place
parameter_types! {
	pub const TransactionByteFee: Balance = 1 * MILLICENTS;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Trait for Test {
    type Currency = Balances;
    type OnTransactionPayment = ();
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate =
    TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

parameter_types! {
    pub const MaxSessionKeysPerAccount: u16 = 2;
    pub const BaseSessionKeyBond: Balance = DEFAULT_SESSION_KEY_BALANCE;
}

impl Trait for Test {
    type Event = ();
    type Call = Call;
    type MaxSessionKeysPerAccount = MaxSessionKeysPerAccount;
    type BaseFilter = ();
    type BaseSessionKeyBond = BaseSessionKeyBond;
}

pub(crate) type System = system::Module<Test>;
pub(crate) type SessionKeys = Module<Test>;
pub(crate) type Balances = pallet_balances::Module<Test>;
type ProfileFollows = pallet_profile_follows::Module<Test>;

pub(crate) type AccountId = u64;
pub(crate) type BlockNumber = u64;
pub(crate) type Balance = u64;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> TestExternalities {
        let storage = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| System::set_block_number(1));

        ext
    }

    pub fn build_with_balance() -> TestExternalities {
        let storage = system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = TestExternalities::from(storage);
        ext.execute_with(|| {
            System::set_block_number(1);

            Balances::make_free_balance_be(&ACCOUNT_MAIN, 10 * DOLLARS);
        });

        ext
    }
}

pub(crate) const ACCOUNT_MAIN: AccountId = 1;
pub(crate) const ACCOUNT_PROXY: AccountId = 2;
pub(crate) const ACCOUNT3: AccountId = 3;
pub(crate) const ACCOUNT4: AccountId = 4;

pub(crate) const DEFAULT_SESSION_KEY_BALANCE: Balance = 1 * DOLLARS;
pub(crate) const BLOCKS_TO_LIVE: BlockNumber = 20;

pub(crate) const fn follow_account_proxy_call() -> Call {
    Call::ProfileFollows(ProfileFollowsCall::follow_account(ACCOUNT_PROXY))
}

pub(crate) fn _add_default_key() -> DispatchResult {
    _add_key(None, None, None, None)
}

pub(crate) fn _add_key(
    origin: Option<Origin>,
    key_account: Option<AccountId>,
    time_to_live: Option<BlockNumber>,
    limit: Option<Option<Balance>>,
) -> DispatchResult {
    SessionKeys::add_key(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT_MAIN)),
        key_account.unwrap_or(ACCOUNT_PROXY),
        time_to_live.unwrap_or(BLOCKS_TO_LIVE),
        limit.unwrap_or(Some(DEFAULT_SESSION_KEY_BALANCE)),
    )
}

pub(crate) fn _remove_default_key() -> DispatchResult {
    _remove_key(None, None)
}

pub(crate) fn _remove_key(
    origin: Option<Origin>,
    key_account: Option<AccountId>,
) -> DispatchResult {
    SessionKeys::remove_key(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT_MAIN)),
        key_account.unwrap_or(ACCOUNT_PROXY),
    )
}

pub(crate) fn _remove_default_keys() -> DispatchResult {
    _remove_keys(None)
}

pub(crate) fn _remove_keys(
    origin: Option<Origin>
) -> DispatchResult {
    SessionKeys::remove_keys(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT_MAIN))
    )
}

pub(crate) fn _default_proxy() -> DispatchResult {
    _proxy(None, None)
}

pub(crate) fn _proxy(
    origin: Option<Origin>,
    call: Option<Call>,
) -> DispatchResult {
    SessionKeys::proxy(
        origin.unwrap_or_else(|| Origin::signed(ACCOUNT_PROXY)),
        Box::new(call.unwrap_or(follow_account_proxy_call())),
    )
}