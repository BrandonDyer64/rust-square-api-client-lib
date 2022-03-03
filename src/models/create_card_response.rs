//! Response struct for the Create Card API

use serde::Deserialize;

use super::errors::Error;
use super::Card;

/// This is a model struct for CreateCardResponse type.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq)]
pub struct CreateCardResponse {
    /// Any errors that occurred during the request.
    pub errors: Vec<Error>,
    /// Represents the payment details of a card to be used for payments. These details are
    /// determined by the payment token generated by Web Payments SDK.
    pub card: Card,
}
