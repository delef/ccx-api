#[cfg(feature = "with_network")]
pub use with_network::*;

use crate::api::spot::SpotApi;
use crate::error::LibResult;
use crate::types::DelIncomingSettlementCPRequest;
use crate::types::DelIncomingSettlementCPResponse;
use crate::types::Nonce;
use crate::types::Time;
use crate::types::API_DEL_INCOMING_SETTLEMENT_CP_REQUEST;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S: crate::client::FinerySigner> SpotApi<S> {
        pub async fn del_incoming_settlement_cp_request(
            &self,
            nonce: Nonce,
            time: Time,
            request: impl Into<DelIncomingSettlementCPRequest>,
        ) -> LibResult<DelIncomingSettlementCPResponse> {
            self.client
                .post(API_DEL_INCOMING_SETTLEMENT_CP_REQUEST)?
                .nonce(nonce)?
                .time(time)?
                .content(request.into())?
                .send()
                .await
        }
    }
}
