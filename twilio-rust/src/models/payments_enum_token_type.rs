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
pub enum PaymentsEnumTokenType {
    #[serde(rename = "one-time")]
    OneTime,
    #[serde(rename = "reusable")]
    Reusable,

}

impl ToString for PaymentsEnumTokenType {
    fn to_string(&self) -> String {
        match self {
            Self::OneTime => String::from("one-time"),
            Self::Reusable => String::from("reusable"),
        }
    }
}

impl Default for PaymentsEnumTokenType {
    fn default() -> PaymentsEnumTokenType {
        Self::OneTime
    }
}



