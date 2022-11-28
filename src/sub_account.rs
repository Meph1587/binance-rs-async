use crate::client::*;
use crate::errors::*;
use crate::util::*;

static API_V1_SUBACCOUNT_SUBTOSUB: &str = "/sapi/v1/sub-account/transfer/subToSub";
static API_V1_SUBACCOUNT_SUBTOMASTER: &str = "/sapi/v1/sub-account/transfer/subToMaster";


/// This struct acts as a gateway for all sub-account endpoints.
/// Preferably use the trait [`Binance`] to get an instance.
#[derive(Clone)]
pub struct SubAccount {
    pub client: Client,
    pub recv_window: u64,
}


/// Sub to Sub Transfer Request
/// transfer asset from sub account or master to sub account
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubToSubTransfer {
    pub to_email: String,
    pub asset: String,
    pub amount: f64,
    /// Cannot be greater than 60000
    pub recv_window: Option<u64>,
}

/// Sub to Master Transfer Request
/// transfer asset from sub account to master account
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubToMasterTransfer {
    pub asset: String,
    pub amount: f64,
    /// Cannot be greater than 60000
    pub recv_window: Option<u64>,
}


impl SubAccount {
    
    pub async fn transfer_subtosub(&self, o: SubToSubTransfer) -> Result<i64> {
        let recv_window = o.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(o, recv_window)?;
        self.client.post_signed_d(API_V1_SUBACCOUNT_SUBTOSUB, &request).await
    }


    pub async fn transfer_subtomaster(&self, o: SubToMasterTransfer) -> Result<i64> {
        let recv_window = o.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(o, recv_window)?;
        self.client.post_signed_d(API_V1_SUBACCOUNT_SUBTOMASTER, &request).await
    }

}
