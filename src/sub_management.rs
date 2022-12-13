use crate::client::*;
use crate::errors::*;

static API_V1_SUBACCOUNT_SUBTOSUB: &str = "/sapi/v1/sub-account/transfer/subToSub";
static API_V1_SUBACCOUNT_SUBTOMASTER: &str = "/sapi/v1/sub-account/transfer/subToMaster";


/// This struct acts as a gateway for all sub-account endpoints.
/// Preferably use the trait [`Binance`] to get an instance.
#[derive(Clone)]
pub struct SubManagement {
    pub client: Client,
    pub recv_window: u64,
}


/// Sub to Sub Transfer Request
/// transfer asset from sub account or master to sub account
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubToSubTransfer {
    pub to_email: String,
    pub asset: String,
    pub amount: f64,
}

/// Sub to Master Transfer Request
/// transfer asset from sub account to master account
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubToMasterTransfer {
    pub asset: String,
    pub amount: f64,
}

/// Sub to Master Transfer Request
/// transfer asset from sub account to master account
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransfer {
    pub from_email: String,
    pub to_email: String,
    pub from_account_type: String,
    pub to_account_type: String,
    pub asset: String,
    pub amount: f64,
}



/// Sub to Master Transfer Request
/// transfer asset from sub account to master account
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub tran_id: i64,
   
}

impl SubManagement {

    
    pub async fn transfer_mastertosub(
        &self, 
        subaccount_email: String,
        asset: String,
        amount: f64
    ) -> Result<i64> {
        let transfer = UniversalTransfer {
            from_email: "chronograph@dialectic.ky".into(),
            to_email: subaccount_email.into(),
            from_account_type: "SPOT".into(),
            to_account_type: "SPOT".into(),
            asset: asset.into(),
            amount: amount.into()
        };
        let resp: Response = self.client
            .post_signed_p("/sapi/v1/sub-account/universalTransfer", transfer, self.recv_window)
            .await?;

        Ok(resp.tran_id)
    }


    pub async fn transfer_subtomaster(
        &self, 
        subaccount_email: String,
        asset: String,
        amount: f64
    ) -> Result<i64> {
        let transfer = UniversalTransfer {
            to_email: "chronograph@dialectic.ky".into(),
            from_email: subaccount_email.into(),
            from_account_type: "SPOT".into(),
            to_account_type: "SPOT".into(),
            asset: asset.into(),
            amount: amount.into()
        };
        let resp:Response = self.client
            .post_signed_p("/sapi/v1/sub-account/universalTransfer", transfer, self.recv_window).await?;

        Ok(resp.tran_id)
        
    
    }

}
