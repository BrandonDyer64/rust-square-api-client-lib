//! Request body struct for the Retrieve Gift Card From Nonce API

use serde::Serialize;

/// This is a model struct for RetrieveGiftCardFromNonceRequest type
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct RetrieveGiftCardFromNonceRequest {
    /// The payment token of the gift card to retrieve. Payment tokens are generated by the Web
    /// Payments SDK or In-App Payments SDK.
    ///
    /// Min Length: 1
    pub nonce: String,
}