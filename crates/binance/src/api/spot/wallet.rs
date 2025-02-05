use super::prelude::*;
use super::RL_WEIGHT_PER_MINUTE;
use crate::client::Task;

pub const SAPI_V1_SYSTEM_STATUS: &str = "/sapi/v1/system/status";
pub const SAPI_V1_CAPITAL_CONFIG_GETALL: &str = "/sapi/v1/capital/config/getall";
// TODO pub const SAPI_V1_ACCOUNT_SNAPSHOT: &str = "/sapi/v1/accountSnapshot";
pub const SAPI_V1_ACCOUNT_DISABLE_FAST_WITHDRAW: &str =
    "/sapi/v1/account/disableFastWithdrawSwitch";
pub const SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW: &str = "/sapi/v1/account/enableFastWithdrawSwitch";
pub const SAPI_V1_CAPITAL_WITHDRAW_APPLY: &str = "/sapi/v1/capital/withdraw/apply";
pub const SAPI_V1_CAPITAL_DEPOSIT_HISTORY: &str = "/sapi/v1/capital/deposit/hisrec";
pub const SAPI_V1_CAPITAL_WITHDRAW_HISTORY: &str = "/sapi/v1/capital/withdraw/history";
pub const SAPI_V1_CAPITAL_DEPOSIT_ADDRESS: &str = "/sapi/v1/capital/deposit/address";
pub const SAPI_V1_ACCOUNT_STATUS: &str = "/sapi/v1/account/status";
pub const SAPI_V1_ACCOUNT_TRADING_STATUS: &str = "/sapi/v1/account/apiTradingStatus";
pub const SAPI_V1_ASSET_DRIBLET: &str = "/sapi/v1/asset/dribblet";
pub const SAPI_V1_ASSET_DUST: &str = "/sapi/v1/asset/dust";
pub const SAPI_V1_ASSET_DIVIDEND: &str = "/sapi/v1/asset/assetDividend";
pub const SAPI_V1_ASSET_DETAIL: &str = "/sapi/v1/asset/assetDetail";
pub const SAPI_V1_ASSET_TRADE_FEE: &str = "/sapi/v1/asset/tradeFee";
pub const SAPI_V1_ASSET_TRANSFER: &str = "/sapi/v1/asset/transfer";
pub const SAPI_V1_ASSET_GET_FUNDING_ASSET: &str = "/sapi/v1/asset/get-funding-asset";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    pub status: SystemMaintenanceStatus,
    pub msg: String,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum SystemMaintenanceStatus {
    Normal = 0,
    SystemMaintenance = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInformation {
    pub coin: Atom,
    pub deposit_all_enable: bool,
    pub free: Decimal,
    pub freeze: Decimal,
    pub ipoable: Decimal,
    pub ipoing: Decimal,
    pub is_legal_money: bool,
    pub locked: Decimal,
    pub name: Atom,
    pub network_list: Vec<NetworkInformation>,
    pub storage: Decimal,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    pub withdrawing: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInformation {
    pub address_regex: Atom,
    pub coin: Atom,
    /// Shown only when "depositEnable" is false.
    pub deposit_desc: Option<Atom>,
    pub deposit_enable: bool,
    pub insert_time: Option<u64>,
    pub is_default: bool,
    pub memo_regex: Atom,
    /// Min number for balance confirmation.
    pub min_confirm: i32,
    pub name: Atom,
    pub network: Atom,
    pub reset_address_status: bool,
    pub special_tips: Option<Atom>,
    /// Confirmation number for balance unlock.
    pub un_lock_confirm: i32,
    pub update_time: Option<u64>,
    /// Shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<Atom>,
    pub withdraw_enable: bool,
    pub withdraw_fee: Decimal,
    pub withdraw_min: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Deposit {
    pub amount: Decimal,
    pub coin: String,
    pub network: String,
    pub status: DepositStatus,
    pub address: String,
    pub address_tag: String,
    /// Returned only when requested with `includeSource` set to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    pub tx_id: String,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub insert_time: u64,
    pub transfer_type: TransferType,
    // pub unlock_confirm: String,
    pub confirm_times: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub address: String,
    pub coin: Atom,
    pub tag: String,
    pub url: String,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum DepositStatus {
    Pending = 0,
    Success = 1,
    Rejected = 2,
    Processing = 6,
    WrongDeposit = 7,
    WaitingForConfirmation = 8,
}

impl DepositStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, DepositStatus::Success)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, DepositStatus::Pending)
    }

    pub fn is_processing(&self) -> bool {
        matches!(self, DepositStatus::Processing)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewWithdraw {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    pub address: String,
    pub amount: Decimal,
    #[serde(default)]
    pub transaction_fee: Decimal,
    // FIXME decode time, example: "2021-04-29 16:08:00"
    pub apply_time: String,
    pub coin: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdraw_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub transfer_type: TransferType,
    pub status: WithdrawStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum WithdrawStatus {
    EmailSent = 0,
    Cancelled = 1,
    AwaitingApproval = 2,
    Rejected = 3,
    Processing = 4,
    Failure = 5,
    Completed = 6,
}

#[derive(
    Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, Ord, PartialOrd, PartialEq, Hash,
)]
#[repr(u32)]
pub enum TransferType {
    External = 0,
    Internal = 1,
}

impl WithdrawStatus {
    pub fn is_finished(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(
            self,
            WS::Completed | WS::Cancelled | WS::Rejected | WS::Failure
        )
    }

    pub fn is_pending(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent | WS::AwaitingApproval | WS::Processing)
    }

    pub fn needs_confirmation(&self) -> bool {
        use WithdrawStatus as WS;
        matches!(self, WS::EmailSent)
    }
}

impl TransferType {
    pub fn is_external(&self) -> bool {
        matches!(self, TransferType::External)
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, TransferType::Internal)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfer {
    #[serde(rename = "tranId")]
    pub transfer_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingAsset {
    pub asset: Atom,
    /// avalible balance.
    pub free: Decimal,
    /// locked asset.
    pub locked: Decimal,
    /// freeze asset.
    pub freeze: Decimal,
    pub withdrawing: Decimal,
    pub btc_valuation: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingStatus {
    pub data: AccountTradingStatusData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradingStatusData {
    pub is_locked: bool,
    pub planned_recover_time: u64,
    pub trigger_condition: HashMap<String, u64>,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDribblet {
    pub total: u64,
    pub user_asset_dribblets: Vec<UserAssetDribblet>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribblet {
    pub operate_time: u64,
    pub total_transfered_amount: Decimal,
    pub total_service_charge_amount: Decimal,
    pub trans_id: u64,
    pub user_asset_dribblet_details: Vec<UserAssetDribbletDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribbletDetails {
    pub trans_id: u64,
    pub service_charge_amount: Decimal,
    pub amount: Decimal,
    pub operate_time: u64,
    pub transfered_amount: Decimal,
    pub from_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDust {
    pub total_service_charge: Decimal,
    pub total_transfered: Decimal,
    pub transfer_result: Vec<AssetDustResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDustResult {
    pub amount: Decimal,
    pub from_asset: String,
    pub operate_time: u64,
    pub service_charge_amount: Decimal,
    pub tran_id: u64,
    pub transfered_amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividend {
    pub rows: Vec<AssetDividendRow>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividendRow {
    id: u64,
    amount: Decimal,
    asset: String,
    div_time: u64,
    en_info: String,
    tran_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    #[serde(flatten)]
    pub asset: HashMap<Atom, AssetDetailInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetailInfo {
    min_withdraw_amount: Decimal,
    deposit_status: bool,
    withdraw_fee: Decimal,
    withdraw_status: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    deposit_tip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeFee {
    pub symbol: String,
    pub maker_commission: Decimal,
    pub taker_commission: Decimal,
}

type NoResponse = HashMap<(), ()>;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::BinanceSigner,
        S: Unpin + 'static,
    {
        /// User Universal Transfer (USER_DATA)
        ///
        /// You need to enable Permits Universal Transfer option for the API Key which requests this endpoint.
        ///
        /// Weight(IP): 1
        pub fn asset_transfer(
            &self,
            transfer_type: TransferKind,
            asset: impl Serialize,
            amount: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Transfer>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_ASSET_TRANSFER)?
                        .signed(time_window)?
                        .query_arg("type", &transfer_type)?
                        .query_arg("asset", &asset)?
                        .query_arg("amount", &amount)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Funding Wallet (USER_DATA)
        ///
        /// Weight(IP): 1
        ///
        /// * Currently supports querying the following business assets：Binance Pay, Binance Card,
        /// Binance Gift Card, Stock Token.
        pub fn asset_fundings(
            &self,
            asset: Option<impl Serialize>,
            need_btc_valuation: Option<bool>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<FundingAsset>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_ASSET_GET_FUNDING_ASSET)?
                        .signed(time_window)?
                        .try_query_arg("asset", &asset)?
                        .try_query_arg("needBtcValuation", &need_btc_valuation)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// System Status (System)
        ///
        /// Fetch system status.
        ///
        /// Weight(IP): 1
        pub fn system_status(&self) -> BinanceResult<Task<SystemStatus>> {
            Ok(self
                .rate_limiter
                .task(self.client.get(SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW)?)
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// All Coins' Information (USER_DATA)
        ///
        /// Get information of coins (available for deposit and withdraw) for user.
        ///
        /// Weight(IP): 10
        pub fn all_coins_information(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<CoinInformation>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_CAPITAL_CONFIG_GETALL)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Disable Fast Withdraw Switch (USER_DATA)
        ///
        /// Weight(IP): 1
        ///
        /// Caution:
        ///
        /// * This request will disable fastwithdraw switch under your account.
        /// * You need to enable "trade" option for the api key which requests this endpoint.
        pub fn disable_fast_withdraw_switch(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<NoResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_ACCOUNT_DISABLE_FAST_WITHDRAW)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Enable Fast Withdraw Switch (USER_DATA)
        ///
        /// Weight(IP): 1
        ///
        /// This request will enable fastwithdraw switch under your account.
        /// You need to enable "trade" option for the api key which requests this endpoint.
        /// When Fast Withdraw Switch is on, transferring funds to a Binance account will be done
        ///   instantly. There is no on-chain transaction, no transaction ID and no withdrawal fee.
        pub fn enable_fast_withdraw_switch(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<NoResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_ACCOUNT_ENABLE_FAST_WITHDRAW)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Deposit Address (supporting network) (USER_DATA)
        ///
        /// Fetch deposit address with network.
        ///
        /// Weight(IP): 10
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
        pub fn get_deposit_address(
            &self,
            coin: impl Serialize,
            network: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<DepositAddress>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_CAPITAL_DEPOSIT_ADDRESS)?
                        .signed(time_window)?
                        .query_arg("coin", &coin)?
                        .try_query_arg("network", &network)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Withdraw(SAPI)
        ///
        /// Submit a withdraw request.
        ///
        /// Weight(IP): 1
        ///
        /// * withdrawOrderId - client id for withdraw
        /// * addressTag - Secondary address identifier for coins like XRP,XMR etc.
        /// * transactionFeeFlag - When making internal transfer, true for returning the fee
        ///     to the destination account; false for returning the fee back to the departure account.
        ///     Default false.
        /// * name - Description of the address. Space in name should be encoded into %20.
        ///
        /// If network is not send, return with default network of the coin.
        /// You can get network and isDefault in networkList in the response of
        ///    Get /sapi/v1/capital/config/getall (HMAC SHA256).
        #[allow(clippy::too_many_arguments)]
        pub fn withdraw(
            &self,
            coin: impl Serialize,
            withdraw_order_id: Option<impl Serialize>,
            network: Option<impl Serialize>,
            address: impl Serialize,
            address_tag: Option<impl Serialize>,
            amount: Decimal,
            transaction_fee_flag: Option<bool>,
            name: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<NewWithdraw>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_CAPITAL_WITHDRAW_APPLY)?
                        .signed(time_window)?
                        .query_arg("coin", &coin)?
                        .try_query_arg("withdrawOrderId", &withdraw_order_id)?
                        .try_query_arg("network", &network)?
                        .query_arg("address", &address)?
                        .try_query_arg("addressTag", &address_tag)?
                        .query_arg("amount", &amount)?
                        .try_query_arg("transactionFeeFlag", &transaction_fee_flag)?
                        .try_query_arg("name", &name)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Deposit History (supporting network) (USER_DATA)
        ///
        /// Fetch deposit history.
        ///
        /// Weight(IP): 1
        ///
        /// * startTime - Default: 90 days from current timestamp
        /// * endTime - Default: present timestamp
        ///
        /// * network may not be in the response for old deposit.
        /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
        #[allow(clippy::too_many_arguments)]
        pub fn deposit_history(
            &self,
            include_source: Option<bool>,
            coin: Option<impl Serialize>,
            status: Option<DepositStatus>,
            offset: Option<u16>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<Deposit>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_CAPITAL_DEPOSIT_HISTORY)?
                        .signed(time_window)?
                        .try_query_arg("includeSource", &include_source)?
                        .try_query_arg("coin", &coin)?
                        .try_query_arg("status", &status)?
                        .try_query_arg("offset", &offset)?
                        .try_query_arg("limit", &limit)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Withdraw History (supporting network) (USER_DATA)
        ///
        /// Fetch withdraw history.
        ///
        /// Weight(IP): 1
        ///
        /// * startTime - Default: 90 days from current timestamp
        /// * endTime - Default: present timestamp
        ///
        /// * network may not be in the response for old withdraw.
        /// * Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        /// * If both startTime and endTime are sent, time between startTime and endTime must be less than 90 days.
        #[allow(clippy::too_many_arguments)]
        pub fn withdraw_history(
            &self,
            coin: Option<impl Serialize>,
            status: Option<WithdrawStatus>,
            offset: Option<u16>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<Withdraw>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_CAPITAL_WITHDRAW_HISTORY)?
                        .signed(time_window)?
                        .try_query_arg("coin", &coin)?
                        .try_query_arg("status", &status)?
                        .try_query_arg("offset", &offset)?
                        .try_query_arg("limit", &limit)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Account Status (USER_DATA)
        ///
        /// Fetch account status detail.
        ///
        /// Weight(IP): 1
        pub fn account_status(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AccountStatus>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ACCOUNT_STATUS)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Account API Trading Status (USER_DATA)
        ///
        /// Fetch account api trading status detail.
        ///
        /// Weight(IP): 1
        pub fn account_trading_status(
            &self,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AccountTradingStatus>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ACCOUNT_TRADING_STATUS)?
                        .signed(time_window)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// DustLog(USER_DATA)
        ///
        /// Weight(IP): 1
        ///
        /// * Only return last 100 records
        /// * Only return records after 2020/12/01
        pub fn asset_dribblet(
            &self,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AssetDribblet>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ASSET_DRIBLET)?
                        .signed(time_window)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Dust Transfer (USER_DATA)
        ///
        /// Convert dust assets to BNB.
        ///
        /// Weight(UID): 10
        ///
        /// * You need to openEnable Spot & Margin Trading permission
        ///   for the API Key which requests this endpoint.
        pub fn asset_dust(
            &self,
            asset: impl Serialize,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AssetDust>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(SAPI_V1_ASSET_DUST)?
                        .signed(time_window)?
                        .query_arg("asset", &asset)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Asset Dividend Record (USER_DATA)
        ///
        /// Query asset dividend record.
        ///
        /// Weight(IP): 10
        pub fn asset_dividend(
            &self,
            asset: Option<impl Serialize>,
            limit: Option<u16>,
            start_time: Option<u64>,
            end_time: Option<u64>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AssetDividend>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ASSET_DIVIDEND)?
                        .signed(time_window)?
                        .try_query_arg("asset", &asset)?
                        .try_query_arg("limit", &limit)?
                        .try_query_arg("startTime", &start_time)?
                        .try_query_arg("endTime", &end_time)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 10)
                .send())
        }

        /// Asset Detail (USER_DATA)
        ///
        /// Fetch details of assets supported on Binance.
        ///
        /// Weight(IP): 1
        ///
        /// * Please get network and other deposit or withdraw details
        ///   from GET /sapi/v1/capital/config/getall.
        pub fn asset_detail(
            &self,
            asset: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<AssetDetail>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ASSET_DETAIL)?
                        .signed(time_window)?
                        .try_query_arg("asset", &asset)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }

        /// Trade Fee (USER_DATA)
        ///
        /// Fetch trade fee
        ///
        /// Weight(IP): 1
        pub fn trade_fee(
            &self,
            symbol: Option<impl Serialize>,
            time_window: impl Into<TimeWindow>,
        ) -> BinanceResult<Task<Vec<TradeFee>>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .get(SAPI_V1_ASSET_TRADE_FEE)?
                        .signed(time_window)?
                        .try_query_arg("symbol", &symbol)?,
                )
                .cost(RL_WEIGHT_PER_MINUTE, 1)
                .send())
        }
    }
}
