/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.54.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentsEnumPaymentMethod {
    #[serde(rename = "credit-card")]
    CreditCard,
    #[serde(rename = "ach-debit")]
    AchDebit,

}

impl ToString for PaymentsEnumPaymentMethod {
    fn to_string(&self) -> String {
        match self {
            Self::CreditCard => String::from("credit-card"),
            Self::AchDebit => String::from("ach-debit"),
        }
    }
}

impl Default for PaymentsEnumPaymentMethod {
    fn default() -> PaymentsEnumPaymentMethod {
        Self::CreditCard
    }
}



